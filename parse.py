from tree_sitter import Language, Parser
import networkx as nx
import tree_sitter_python as tspython
import draw_graph
import re
import json
# PY_LANGUAGE = Language(tspython.language())
from tree_sitter_languages import get_language, get_parser

class CodeNode:
    def __init__(self, qname: str, ntype: str, code: str, filename: str, lineno: int):
        self.name = qname              # 全限定名，如 "A.__init__"
        self.type = ntype              # 'function' or 'class'
        self.code = code               # 原始源码片段
        self.filename = filename       # 源文件路径
        self.lineno = lineno           # 定义所在行号
        # IR 字段
        self.params = []               # 参数列表 [(name, type)]
        self.return_type = None        # 返回类型
        self.fields = []               # 类字段列表 [(name, type)]
        self.bases = []                # 继承基类列表
        self.body_ir = []              # 简化的函数体 IR
        self.num_lines = 0             # 代码行数
        self.rust_translation = None

    def __repr__(self):
        return f"CodeNode({self.name})"

    def __hash__(self):
        return hash(self.name)

    def __eq__(self, other):
        return isinstance(other, CodeNode) and self.name == other.name

class IRBuilder:

    @staticmethod
    def build_body_ir(node: CodeNode):
        node.body_ir = node.code.splitlines()

    @staticmethod
    def build_num_lines(node: CodeNode):
        node.num_lines = len(node.body_ir)

    @staticmethod
    def generate_ir(nodes_map, order):
        # Merge self.xxx fields into class CodeNode
        for node in nodes_map.values():
            if node.type == 'function' and '.' in node.name:
                cls_name = node.name.rsplit('.', 1)[0]
                class_node = nodes_map.get(cls_name)
                if class_node:
                    for line in node.body_ir or node.code.splitlines():
                        m = re.match(r"\s*self\.(\w+)\s*=", line)
                        if m:
                            field = m.group(1)
                            if field not in class_node.fields:
                                class_node.fields.append(field)

        ir_nodes = []
        for node in order:
            IRBuilder.build_body_ir(node)
            IRBuilder.build_num_lines(node)

            # Determine enclosing class for functions
            class_name = node.name.rsplit('.', 1)[0] if node.type == 'function' and '.' in node.name else None

            ir_entry = {
                'qname': node.name,
                'kind': node.type,
                'filename': node.filename,
                'lineno': node.lineno,
                'body_ir': node.body_ir,
                'num_lines': node.num_lines,
            }

            if node.type == 'class':
                ir_entry['fields'] = node.fields
                ir_entry['base_class'] = node.bases
            if node.type == 'function':
                ir_entry['class'] = class_name

            ir_nodes.append(ir_entry)
        return ir_nodes

# —— 2) TSDepGraph：用 CodeNode 替代字符串节点 —— 
class TSDepGraph:
    def __init__(self):
        self.parser      = get_parser("python")
        self.graph = nx.DiGraph()
        self.nodes_map = {}   # nodename -> CodeNode
        self.calls = []       # (caller_node, callee_name)
        self.inherits = []    # (subclass_name, base_name)
        self.scope_stack = []
        self.class_stack = []
        self.current = None   # 当前作用域的 CodeNode
        self.alias_map = {}  # alias -> fully qualified module or name
        self.current_file = None



    def add_def(self, qname: str, ntype: str, code: str, node):
        filename = self.current_file
        lineno = node.start_point[0] + 1
        if qname not in self.nodes_map:
            cn = CodeNode(qname, ntype, code, filename, lineno)
            self.nodes_map[qname] = cn
            self.graph.add_node(cn)
        return self.nodes_map[qname]

    def traverse(self, node, src_bytes):
        kind = node.type

        # Class definition
        if kind == 'class_definition':
            if self.current and self.current.type == 'function':
                return

            name_node = node.child_by_field_name('name')
            cls_name = src_bytes[name_node.start_byte:name_node.end_byte].decode()
            qname = '.'.join(self.scope_stack + [cls_name])

            # Capture inheritance
            base_list = []
            for child in node.children:
                if child.type == 'argument_list':
                    for arg in child.named_children:
                        if arg.type in ('identifier', 'attribute'):
                            base = src_bytes[arg.start_byte:arg.end_byte].decode()
                            base_list.append(base)
                            self.inherits.append((qname, base))

            # Build class code snippet
            body = node.child_by_field_name('body')
            header = src_bytes[node.start_byte:body.start_byte].decode().rstrip()
            parts = [header]
            for stmt in body.children:
                if stmt.type == 'assignment':
                    parts.append(src_bytes[stmt.start_byte:stmt.end_byte].decode().rstrip())
                elif stmt.type == 'function_definition':
                    fn_body = stmt.child_by_field_name('body')
                    sig = src_bytes[stmt.start_byte:fn_body.start_byte].decode().rstrip()
                    parts.append(sig + '  # ...')
            class_code = '\n'.join(parts)

            class_node = self.add_def(qname, 'class', class_code, node)
            class_node.bases = base_list

            # Enter class scope
            self.scope_stack.append(cls_name)
            self.class_stack.append(cls_name)
            prev, self.current = self.current, class_node
            for c in node.children:
                self.traverse(c, src_bytes)
            self.current = prev
            self.scope_stack.pop()
            self.class_stack.pop()
            return

        # Function definition
        if kind == 'function_definition':
            if self.current and self.current.type == 'function':
                return

            name_node = node.child_by_field_name('name')
            fn_name = src_bytes[name_node.start_byte:name_node.end_byte].decode()
            qname = '.'.join(self.scope_stack + [fn_name])

            fn_code = src_bytes[node.start_byte:node.end_byte].decode()
            fn_node = self.add_def(qname, 'function', fn_code, node)

            if self.current and self.current.type == 'class':
                self.graph.add_edge(fn_node, self.current)

            # Enter function scope
            self.scope_stack.append(fn_name)
            prev, self.current = self.current, fn_node
            for c in node.children:
                self.traverse(c, src_bytes)
            self.current = prev
            self.scope_stack.pop()
            return

        # Calls
        if kind == 'call' and self.current:
            fn = node.child_by_field_name('function')
            if fn and fn.type == 'attribute':
                obj = fn.child_by_field_name('object')
                attr = fn.child_by_field_name('attribute')
                obj_name = src_bytes[obj.start_byte:obj.end_byte].decode()
                method = src_bytes[attr.start_byte:attr.end_byte].decode()
                if obj_name == 'self' and self.class_stack:
                    cls = self.class_stack[-1]
                    qn = f"{cls}.{method}"
                    tgt = self.nodes_map.get(qn)
                    if tgt:
                        self.graph.add_edge(self.current, tgt)
                        return
            if fn and fn.type in ('identifier', 'attribute'):
                callee = src_bytes[fn.start_byte:fn.end_byte].decode()
                self.calls.append((self.current, callee))

        # Recurse children
        for c in node.children:
            self.traverse(c, src_bytes)

    def build_edges(self):
        for caller, callee in self.calls:
            tgt = self.nodes_map.get(callee)
            if tgt:
                self.graph.add_edge(caller, tgt)

    def build_ir(self, order):
        return IRBuilder.generate_ir(self.nodes_map, order)


def build_files(folder, filenames,draw_graph_flag=False):
    dg = TSDepGraph()
    for path in filenames:
        dg.current_file = path
        src = open(folder + path, 'rb').read()
        tree = dg.parser.parse(src)
        dg.traverse(tree.root_node, src)
    dg.build_edges()
    dg.graph.remove_edges_from(nx.selfloop_edges(dg.graph))

    order = list(nx.topological_sort(dg.graph))
    order = order[::-1]
    print("翻译顺序：", order)


    if draw_graph_flag:
        draw_graph.draw_original(dg.graph)
        draw_graph.draw_min_crossing(dg.graph)
    ir = dg.build_ir(order)
    with open('middle_files/pre_ir.json', 'w', encoding='utf-8') as f:
        json.dump(ir, f, ensure_ascii=False, indent=2)
    print("IR 已输出到 middle_files/pre_ir.json")
    return dg.graph, order

if __name__ == '__main__':
    build_files('annotated_toml_module/', ['decoder.py', 'tz.py'],draw_graph_flag=True)

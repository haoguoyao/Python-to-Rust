import ast
import json
import sys

def parse_stub(stub_path):

    with open(stub_path, 'r', encoding='utf-8') as f:
        source = f.read()
    module = ast.parse(source, stub_path)
    stub_map = {}

    # Helper to unparse annotation or return None
    def ann_to_str(node):
        return ast.unparse(node) if node is not None else None

    # Top-level functions
    for node in module.body:
        if isinstance(node, ast.FunctionDef):
            params = []
            for arg in node.args.args:
                annotation = ann_to_str(arg.annotation)
                params.append((arg.arg, annotation))
            returns = ann_to_str(node.returns)
            stub_map[node.name] = {'params': params, 'return_type': returns}

        elif isinstance(node, ast.ClassDef):
            class_name = node.name
            for item in node.body:
                if isinstance(item, ast.FunctionDef):
                    params = []
                    for arg in item.args.args:
                        annotation = ann_to_str(arg.annotation)
                        params.append((arg.arg, annotation))
                    returns = ann_to_str(item.returns)
                    key = f"{class_name}.{item.name}"
                    stub_map[key] = {'params': params, 'return_type': returns}

    return stub_map

def merge_types(ir_path, stub_path, output_path):
    # Load existing IR
    with open(ir_path, 'r', encoding='utf-8') as f:
        ir_list = json.load(f)

    # Parse stub for types
    stub_map = parse_stub(stub_path)

    # Merge stub types into IR entries
    merged = []
    for ir in ir_list:
        qname = ir.get('qname')
        if qname in stub_map:
            info = stub_map[qname]
            print(f"Merging stub types for {qname}")
            ir['params'] = info['params']
            ir['return_type'] = info['return_type']
            print(ir['params'])
            print(ir['return_type'])
        merged.append(ir)

    # Write merged JSON
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(merged, f, ensure_ascii=False, indent=2)

    print(f"Merged IR with stub types written to {output_path}")

if __name__ == '__main__':

    merge_types('middle_files/pre_ir.json', 'middle_files/decoder.pyi','middle_files/final_ir.json')
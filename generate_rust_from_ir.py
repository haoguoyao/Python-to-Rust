import os
import json
from langchain_openai import ChatOpenAI
from langchain_core.prompts import PromptTemplate
from parse import build_files  
from dotenv import load_dotenv
load_dotenv()
import re
from util import generate_cargo_toml

CLASS_PROMPT = PromptTemplate(
    input_variables=[
        "stub",
        "qname",
        "fields",
        "bases",
        "dependencies",
        "rust_accumulated",
        "implemented_functions"
    ],
    template="""
You are a Rust expert. Your task is to convert Python classes to Rust structs. Do not implement the methods, only the structs.

You have the following stub signatures:
```python
{stub}
```

You have already generated:
```rust
{rust_accumulated}
```

The following functions have already been implemented, DO NOT implement these again:
{implemented_functions}

Translate this Python class to Rust, considering its dependencies:
```json
{dependencies}
```

Class: {qname}
Fields: {fields}
Base classes: {bases}

Follow these strict guidelines:

1. Use proper Rust types:
   - Use String instead of str for owned strings
   - Use &str for string references
   - Use Vec<T> for lists
   - Use HashMap<K, V> for dictionaries
   - Use Option<T> for nullable values
   - Use Result<T, E> for error handling

2. Follow Rust naming conventions:
   - Use snake_case for variables and functions
   - Use CamelCase for types
   - Use SCREAMING_SNAKE_CASE for constants

3. Implement proper error handling:
   - Use Result<T, E> for functions that can fail
   - Use thiserror for custom error types
   - Use ? operator for error propagation

4. Avoid Python-specific patterns:
   - Don't use Python-style indexing
   - Don't use Python-style string operations
   - Don't use Python-style type conversions

5. Use proper Rust ownership rules:
   - Use references (&) for borrowed values
   - Use owned types for owned values
   - Use Clone when necessary

6. Use proper Rust collections:
   - Use Vec<T> instead of Python lists
   - Use HashMap<K, V> instead of Python dicts
   - Use Option<T> instead of None

7. Use proper Rust string handling:
   - Use String for owned strings
   - Use &str for string references
   - Use to_string() for string conversion
   - Use parse() for string parsing

8. Use proper Rust error handling:
   - Use Result<T, E> for functions that can fail
   - Use thiserror for custom error types
   - Use ? operator for error propagation

9. Use proper Rust type conversions:
   - Use parse() for string parsing
   - Use to_string() for string conversion
   - Use as for type casting
   - Use into() for type conversion

10. Use proper Rust collections:
    - Use Vec<T> instead of Python lists
    - Use HashMap<K, V> instead of Python dicts
    - Use Option<T> instead of None

Generate ONLY the Rust code, no explanations.
"""
)

# Prompt for translating functions
FUNCTION_PROMPT = PromptTemplate(
    input_variables=[
        "stub",
        "qname",
        "method_name",
        "class_name",
        "params",
        "return_type",
        "body_ir",
        "dependencies",
        "rust_accumulated",
        "implemented_functions",
    ],
    template="""
You are a Rust expert. Your task is to convert a Python function to Rust functions.
Only implement the target function, do not implement other functions.

You have the following stub signatures:
```python
{stub}
```

You have already generated:
```rust
{rust_accumulated}
```

The following functions have already been implemented, DO NOT implement these again:
{implemented_functions}

Translate this Python function to Rust, considering its dependencies:
```json
{dependencies}
```

Function: {qname}
Enclosing class: {class_name}
Parameters: {params}
Return type: {return_type}
Body IR:
{body_ir}

Follow these strict guidelines:

1. Use proper Rust types:
   - Use String instead of str for owned strings
   - Use &str for string references
   - Use Vec<T> for lists
   - Use HashMap<K, V> for dictionaries
   - Use Option<T> for nullable values
   - Use Result<T, E> for error handling

2. Follow Rust naming conventions:
   - Use snake_case for variables and functions
   - Use CamelCase for types
   - Use SCREAMING_SNAKE_CASE for constants

3. Implement proper error handling:
   - Use Result<T, E> for functions that can fail
   - Use thiserror for custom error types
   - Use ? operator for error propagation

4. Avoid Python-specific patterns:
   - Don't use Python-style indexing
   - Don't use Python-style string operations
   - Don't use Python-style type conversions

5. Use proper Rust ownership rules:
   - Use references (&) for borrowed values
   - Use owned types for owned values
   - Use Clone when necessary

6. Use proper Rust collections:
   - Use Vec<T> instead of Python lists
   - Use HashMap<K, V> instead of Python dicts
   - Use Option<T> instead of None

7. Use proper Rust string handling:
   - Use String for owned strings
   - Use &str for string references
   - Use to_string() for string conversion
   - Use parse() for string parsing

8. Use proper Rust error handling:
   - Use Result<T, E> for functions that can fail
   - Use thiserror for custom error types
   - Use ? operator for error propagation

9. Use proper Rust type conversions:
   - Use parse() for string parsing
   - Use to_string() for string conversion
   - Use as for type casting
   - Use into() for type conversion

10. Use proper Rust collections:
    - Use Vec<T> instead of Python lists
    - Use HashMap<K, V> instead of Python dicts
    - Use Option<T> instead of None

If `class_name` is not null, wrap in:
```rust
impl {class_name} {{
    /// Documentation comment
    pub fn {method_name}(/* ... */) -> Result<ReturnType, ErrorType> {{
        // ...
    }}
}}
```
Otherwise, emit a top-level:
```rust
/// Documentation comment
pub fn {method_name}(/* ... */) -> Result<ReturnType, ErrorType> {{
    // ...
}}
```

Generate ONLY the Rust code, no explanations.
"""
)

# 添加一个函数用于修复常见的Rust代码问题
def fix_common_rust_issues(code):
    """修复常见的Rust代码问题"""
    # 修复字符串索引问题
    code = re.sub(r'(\w+)\[(\w+)\]', r'\1.chars().nth(\2).unwrap_or_default()', code)
    
    # 修复类型转换
    code = re.sub(r'(\w+)\.parse\(\)', r'\1.parse().map_err(|e| TomlDecodeError::new(format!("Parse error: {}", e)))?', code)
    
    # 修复错误处理
    code = re.sub(r'unwrap\(\)', r'?', code)
    
    # 修复集合类型
    code = re.sub(r'Vec::new\(\)', r'Vec::new()', code)
    code = re.sub(r'HashMap::new\(\)', r'HashMap::new()', code)
    
    # 修复字符串处理
    code = re.sub(r'to_string\(\)', r'to_string()', code)
    code = re.sub(r'to_owned\(\)', r'to_string()', code)
    
    # 修复Option处理
    code = re.sub(r'None', r'None', code)
    code = re.sub(r'Some\((.*?)\)', r'Some(\1)', code)
    
    return code

def extract_function_definitions(code):
    """从代码中提取函数定义"""
    # 查找函数和方法定义
    fn_pattern = r'(?:pub\s+)?fn\s+(\w+)\s*\('
    impl_pattern = r'impl\s+(\w+)\s*\{'
    
    functions = []
    impls = {}
    
    # 查找独立函数
    for match in re.finditer(fn_pattern, code):
        fn_name = match.group(1)
        if fn_name not in functions:
            functions.append(fn_name)
    
    # 查找impl块
    current_impl = None
    for line in code.splitlines():
        impl_match = re.search(impl_pattern, line)
        if impl_match:
            current_impl = impl_match.group(1)
            impls[current_impl] = []
        elif current_impl and re.search(fn_pattern, line):
            fn_match = re.search(fn_pattern, line)
            fn_name = fn_match.group(1)
            impls[current_impl].append(fn_name)
    
    return functions, impls

BASE_CODE_PROMPT = PromptTemplate(
    input_variables=["dependencies"],
    template="""
Generate the base Rust code for a TOML parser library. Include necessary imports, error types, and basic structures.

Dependencies to consider:
```json
{dependencies}
```

Follow these guidelines:
1. Include all necessary imports for a TOML parser library
2. Define basic error types
3. Set up type aliases
4. Add basic documentation

Generate ONLY the code, no explanations. The code should be ready to use as a base for the TOML parser implementation.
"""
)


def translate_to_rust(
    ir_path: str,
    stub_path: str,
    folder: str,
    filenames: list,
    output_path: str,
    LLM_model: str = "gpt-4o-mini"
):
    # 1) Build graph and node order
    G, order_nodes = build_files(folder, filenames)

    # 2) Load IR JSON
    with open(ir_path, 'r', encoding='utf-8') as f:
        ir_list = json.load(f)
    ir_map = {entry['qname']: entry for entry in ir_list}

    # 3) Load stub file content
    with open(stub_path, 'r', encoding='utf-8') as sf:
        stub_content = sf.read()

    # 4) Initialize LLM and chains using the new RunnableSequence approach
    llm = ChatOpenAI(model=LLM_model, temperature=0)
    base_chain = BASE_CODE_PROMPT | llm
    class_chain = CLASS_PROMPT | llm
    fn_chain = FUNCTION_PROMPT | llm

    # 5) Prepare a mapping from qname → CodeNode
    node_map = {node.name: node for node in G.nodes()}

    # 跟踪已实现的函数和方法
    implemented_functions = []
    implemented_methods = {}
    
    # 创建输出目录
    if not output_path.endswith('/'):
        output_path = output_path + '/'
    os.makedirs(output_path, exist_ok=True)
    
    # 创建src目录
    src_dir = os.path.join(output_path, "src")
    os.makedirs(src_dir, exist_ok=True)
    parser_path = os.path.join(src_dir, "toml.rs")

    # 6) Generate base code
    print("Generating base code...")
    base_deps = [ir_map[d] for d in node_map.keys() if d in ir_map]
    base_code = base_chain.invoke({
        "dependencies": json.dumps(base_deps, ensure_ascii=False, indent=2)
    })
    
    if hasattr(base_code, "content"):
        base_code = base_code.content
        
    # Clean the base code
    base_code = clean_code(base_code)

    # 6) Generate implementation code first
    print("Generating implementation code...")
    
    # 收集所有的结构体和函数定义
    structs_code = ""
    functions_code = ""
    
    # 第一阶段：处理所有结构体定义
    struct_nodes = [node for node in order_nodes if node.type == 'class']
    for node in struct_nodes:
        qname = node.name
        print(f"Processing struct {qname}...")
        ir = ir_map.get(qname)
        if not ir:
            print(f"  ⚠️ IR not found for {qname}, skipping.")
            continue

        # Build dependency context
        preds = list(G.predecessors(node))
        succs = list(G.successors(node))
        deps = [n.name for n in preds + succs]
        dep_irs = [ir_map[d] for d in deps if d in ir_map]
        dep_text = json.dumps(dep_irs, ensure_ascii=False, indent=2)
        
        # 获取已实现的函数列表
        implemented_list = "\n".join([
            f"- {func}" for func in implemented_functions
        ])
        
        # 生成结构体代码
        code = class_chain.invoke({
            "stub": stub_content,
            "qname": qname,
            "fields": ir.get('fields', []),
            "bases": ir.get('bases', []),
            "dependencies": dep_text,
            "rust_accumulated": structs_code,
            "implemented_functions": implemented_list
        })

        # Extract content from ChatModel output
        if hasattr(code, "content"):
            code = code.content
            
        # 修复常见代码问题
        code = fix_common_rust_issues(code)
        
        # 提取新定义的函数
        fn_defs, impl_blocks = extract_function_definitions(code)
        implemented_functions.extend(fn_defs)
        for impl_name, methods in impl_blocks.items():
            if impl_name not in implemented_methods:
                implemented_methods[impl_name] = []
            implemented_methods[impl_name].extend(methods)
            
        # Store this node's translation in the CodeNode
        node.rust_translation = code
        structs_code += "\n\n" + code

    # 第二阶段：处理所有函数定义
    function_nodes = [node for node in order_nodes if node.type == 'function']
    for node in function_nodes:
        qname = node.name
        print(f"Processing function {qname}...")
        ir = ir_map.get(qname)
        if not ir:
            print(f" IR not found for {qname}, skipping.")
            continue

        # 如果是类方法，检查是否已经在第一阶段实现
        method_name = qname.rsplit('.', 1)[-1]
        class_name = ir.get('class')
        
        # 如果此方法已经实现，则跳过
        if class_name and class_name in implemented_methods and method_name in implemented_methods[class_name]:
            print(f"Method {method_name} already implemented in {class_name}, skipping.")
            continue
            
        # 同样，如果是顶级函数且已实现，也跳过
        if not class_name and method_name in implemented_functions:
            print(f"Function {method_name} already implemented, skipping.")
            continue

        # Build dependency context
        preds = list(G.predecessors(node))
        succs = list(G.successors(node))
        deps = [n.name for n in preds + succs]
        dep_irs = [ir_map[d] for d in deps if d in ir_map]
        dep_text = json.dumps(dep_irs, ensure_ascii=False, indent=2)
        
        # 获取已实现的函数列表
        implemented_list = "\n".join([
            f"- {func}" for func in implemented_functions
        ])
        if class_name in implemented_methods:
            implemented_list += "\n" + "\n".join([
                f"- {class_name}::{meth}" for meth in implemented_methods[class_name]
            ])
        
        body_text = "\n".join(ir.get('body_ir', []))
        code = fn_chain.invoke({
            "stub": stub_content,
            "qname": qname,
            "method_name": method_name,
            "class_name": class_name,
            "params": ir.get('params', []),
            "return_type": ir.get('return_type'),
            "body_ir": body_text,
            "dependencies": dep_text,
            "rust_accumulated": structs_code + functions_code,
            "implemented_functions": implemented_list
        })

        # Extract content from ChatModel output
        if hasattr(code, "content"):
            code = code.content
            
        # 修复常见代码问题
        code = fix_common_rust_issues(code)
        
        # 提取新定义的函数
        fn_defs, impl_blocks = extract_function_definitions(code)
        implemented_functions.extend(fn_defs)
        for impl_name, methods in impl_blocks.items():
            if impl_name not in implemented_methods:
                implemented_methods[impl_name] = []
            implemented_methods[impl_name].extend(methods)
            
        # Store this node's translation in the CodeNode
        node.rust_translation = code
        functions_code += "\n\n" + code

    # 7) Combine all code
    print("Combining code...")
    implementation_code = structs_code + functions_code
    
    # 直接使用base_code和实现代码
    base_code +='''
//base_code end
'''
    all_rust_code = base_code+implementation_code


    # Clean the final code
    all_rust_code = clean_code(all_rust_code)

    # 写入Rust代码到parser.rs
    with open(parser_path, 'w', encoding='utf-8') as outf:
        outf.write(all_rust_code)

    print(f" Rust code written to {parser_path}")
    generate_cargo_toml(output_path)

    return G
    


def clean_code(code):
    """Remove markdown code block markers and other artifacts from the code."""
    # Remove opening ```rust or ``` markers
    code = re.sub(r'^```(?:rust)?\s*\n', '', code, flags=re.MULTILINE)
    # Remove closing ``` markers
    code = re.sub(r'\n```\s*$', '', code, flags=re.MULTILINE)
    # Remove any trailing markdown/artifacts
    code = re.sub(r'%\s*$', '', code)
    # Ensure proper ending
    if not code.endswith('\n'):
        code += '\n'
    return code

if __name__ == '__main__':
    translate_to_rust(
        ir_path='middle_files/final_ir.json',
        stub_path='middle_files/decoder.pyi',
        folder='annotated_toml_module/',
        filenames=['decoder.py', 'tz.py'],
        output_path='rust_output',
        LLM_model="gpt-4o-mini"
    )
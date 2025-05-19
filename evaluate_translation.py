import os
import json
import networkx as nx
import numpy as np
import matplotlib.pyplot as plt
from collections import Counter
import tree_sitter_python as tspython
from tree_sitter_languages import get_parser

from generate_rust_from_ir import translate_to_rust

python_to_common = {
    "function_definition": "function",
    "def": "function",
    "class_definition": "class",
    "return_statement": "return",
    "assignment": "assignment",
    "augmented_assignment": "assignment",
    "call": "call",
    "argument_list": "arguments",
    "attribute": "attribute",
    "expression_statement": "expression_statement",
    "if_statement": "if",
    "elif_clause": "elif",
    "else_clause": "else",
    "for_statement": "for",
    "while_statement": "while",
    "break_statement": "break",
    "continue_statement": "continue",
    "pass_statement": "pass",
    "raise_statement": "raise",
    "try_statement": "try",
    "except_clause": "except",
    "import_statement": "import",
    "typed_parameter": "parameter",
    "typed_default_parameter": "parameter",
    "default_parameter": "parameter",
    "pattern_list": "pattern_list",
    "binary_operator": "binary_operator",
    "unary_operator": "unary_operator",
    "comparison_operator": "comparison_operator",
    "subscript": "subscript",
    "slice": "slice",
    "list_comprehension": "list_comprehension",
    "parenthesized_expression": "parenthesized_expression",
    "conditional_expression": "conditional_expression",
    "not_operator": "not_operator",
    "is not": "comparison_operator",
    "not in": "comparison_operator",
    "is": "comparison_operator",
    "list": "list",
    "dictionary": "dictionary",
    "concatenated_string": "string",
    "string_start": "string",
    "string_content": "string",
    "string_end": "string",
    "escape_sequence": "escape_sequence",
    "boolean_operator": "boolean_operator",
    "none": "none",
    "true": "boolean_literal",
    "false": "boolean_literal"
    # 其它如有再补充
}
rust_to_common = {
    "function_item": "function",
    "fn": "function",
    "struct_item": "struct",
    "enum_item": "enum",
    "trait_item": "trait",
    "return_expression": "return",
    "assignment_expression": "assignment",
    "let_declaration": "assignment",
    "call_expression": "call",
    "arguments": "arguments",
    "field_expression": "attribute",
    "expression_statement": "expression_statement",
    "if_expression": "if",
    "else_clause": "else",
    "for_expression": "for",
    "while_expression": "while",
    "break_expression": "break",
    "continue_expression": "continue",
    "macro_invocation": "macro_invocation",
    "try_expression": "try",
    "match_expression": "match",
    "block": "block",
    "parameter": "parameter",
    "self_parameter": "parameter",
    "tuple_expression": "tuple",
    "tuple_struct_pattern": "tuple",
    "tuple_pattern": "tuple",
    "reference_expression": "reference_expression",
    "attribute_item": "attribute",
    "attribute": "attribute",
    "scoped_identifier": "scoped_identifier",
    "scoped_type_identifier": "scoped_identifier",
    "range_expression": "range_expression",
    "index_expression": "subscript",
    "unit_type": "unit_type",
    "generic_type": "generic_type",
    "primitive_type": "primitive_type",
    "binary_expression": "binary_operator",
    "unary_expression": "unary_operator",
    "comparison_operator": "comparison_operator",
    "boolean_literal": "boolean_literal",
    "integer_literal": "integer_literal",
    "string_literal": "string",
    "char_literal": "char_literal",
    "compound_assignment_expr": "assignment",
    "closure_expression": "closure",
    "closure_parameters": "closure_parameters",
    "type_identifier": "type_identifier",
    "dynamic_type": "dynamic_type",
    "mutable_specifier": "mutable_specifier",
    "field_identifier": "field_identifier",
    "token_tree": "token_tree",
    "line_comment": "comment",
    "ordered_field_declaration_list": "field_declaration_list"
}
def normalize_node_types(node_types, lang, both_keys):
    # both_keys: py_keys & rs_keys
    mapping = python_to_common if lang == "python" else rust_to_common
    return [t if t in both_keys else mapping.get(t, t) for t in node_types]

def setup_tree_sitter():
    python_parser = get_parser("python")
    rust_parser = get_parser("rust")
    return python_parser, rust_parser

def parse_code(parser, code):
    if not code:
        return None
    if isinstance(code, str):
        code = code.encode('utf-8')
    tree = parser.parse(code)
    return tree.root_node

def extract_node_types(node, node_types=None):
    if node_types is None:
        node_types = []
    node_types.append(node.type)
    for child in node.children:
        extract_node_types(child, node_types)
    return node_types

def normalize_node_types(node_types, lang, both_keys):
    mapping = python_to_common if lang == "python" else rust_to_common
    return [t if t in both_keys else mapping.get(t, t) for t in node_types]

def calculate_similarity(python_node_types, rust_node_types, all_types):
    if not python_node_types or not rust_node_types:
        return 0.0
    python_counter = Counter(python_node_types)
    rust_counter = Counter(rust_node_types)
    if all_types is None:
        print("all_types is None")
        return 0.0
    all_types = list(all_types)
    python_vector = np.array([python_counter.get(t, 0) for t in all_types])
    rust_vector = np.array([rust_counter.get(t, 0) for t in all_types])
    # Normalize
    if np.sum(python_vector) > 0:
        python_vector = python_vector / np.sum(python_vector)
    if np.sum(rust_vector) > 0:
        rust_vector = rust_vector / np.sum(rust_vector)
    dot_product = np.dot(python_vector, rust_vector)
    python_norm = np.linalg.norm(python_vector)
    rust_norm = np.linalg.norm(rust_vector)
    if python_norm == 0 or rust_norm == 0:
        return 0.0
    similarity = dot_product / (python_norm * rust_norm)
    return similarity

def main(LLM_model="gpt-4o-mini", output_file="evaluation_results.json"):
    python_parser, rust_parser = setup_tree_sitter()
    G = translate_to_rust(
        ir_path='middle_files/final_ir.json',
        stub_path='middle_files/decoder.pyi',
        folder='annotated_toml_module/',
        filenames=['decoder.py', 'tz.py'],
        output_path='rust_output',
        LLM_model=LLM_model
    )
    results = {"overall_similarity": 0.0, "nodes": []}
    total_similarity = 0.0
    valid_nodes = 0
    order_nodes = list(nx.topological_sort(G))

    # First pass: gather all node types
    py_keys = set()
    rs_keys = set()
    nodes_data = []

    for node in order_nodes:
        python_code = node.code if hasattr(node, 'code') else ""
        rust_code = node.rust_translation if hasattr(node, 'rust_translation') else ""
        if python_code and rust_code:
            python_ast = parse_code(python_parser, python_code)
            rust_ast = parse_code(rust_parser, rust_code)
            if python_ast is None or rust_ast is None:
                continue
            python_node_types = extract_node_types(python_ast)
            rust_node_types = extract_node_types(rust_ast)
            py_keys.update(python_node_types)
            rs_keys.update(rust_node_types)
            nodes_data.append({
                "name": node.name,
                "type": node.type,
                "python_node_types": python_node_types,
                "rust_node_types": rust_node_types
            })

    # 交集
    both_keys = py_keys & rs_keys
    all_types = sorted((py_keys | rs_keys) | set(python_to_common.values()) | set(rust_to_common.values()))

    # Second pass: normalize with both_keys
    for item in nodes_data:
        py_norm = normalize_node_types(item["python_node_types"], lang="python", both_keys=both_keys)
        rs_norm = normalize_node_types(item["rust_node_types"], lang="rust", both_keys=both_keys)
        similarity = calculate_similarity(py_norm, rs_norm, all_types=all_types)
        node_result = {
            "name": item["name"],
            "type": item["type"],
            "similarity": similarity
        }
        results["nodes"].append(node_result)
        total_similarity += similarity
        valid_nodes += 1

    results["overall_similarity"] = total_similarity / valid_nodes if valid_nodes > 0 else 0.0
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2)
    generate_visualization(results, "evaluation_visualization.png")
    print(f"Evaluation completed. Results saved to {output_file}")
    print(f"Overall similarity score: {results['overall_similarity']:.2f}")
    return results

def generate_visualization(results, output_file):
    node_names = [node["name"] for node in results["nodes"]]
    similarities = [node["similarity"] for node in results["nodes"]]
    plt.figure(figsize=(12, 7))
    bars = plt.bar(range(len(node_names)), similarities)
    plt.xticks(range(len(node_names)), node_names, rotation=45, ha="right")
    plt.title("Semantic Similarity by Node")
    plt.ylim([0, 1.2])
    overall = results['overall_similarity']
    top_similar = sorted(results["nodes"], key=lambda x: x["similarity"], reverse=True)[:3]
    summary_text = f"Overall Similarity: {overall:.2f}\n\nTop 3 Most Similar:\n"
    for i, node in enumerate(top_similar):
        summary_text += f"{i+1}. {node['name']}: {node['similarity']:.2f}\n"
    plt.gca().text(
        0.99, 0.98, summary_text, 
        ha='right', va='top', fontsize=12, 
        bbox=dict(boxstyle="round,pad=0.4", fc="wheat", ec="gray", lw=1),
        transform=plt.gca().transAxes
    )
    plt.tight_layout()
    plt.savefig(output_file)
    print(f"Visualization saved to {output_file}")

if __name__ == "__main__":
    main(LLM_model="gpt-4.1", output_file="evaluation_results.json")
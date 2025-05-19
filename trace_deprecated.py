import os
import re
import sys
import inspect
import json
import atexit
import toml

# 全局结构
call_records = []
call_map = {}

def serialize(obj):
    """
    只保留类型信息：返回 obj.__class__.__name__。
    """
    return obj.__class__.__name__

def trace_calls(frame, event, arg):
    if event not in ('call', 'return'):
        return trace_calls

    modname = frame.f_globals.get('__name__', '')
    if not modname.startswith('toml'):
        return trace_calls

    key = id(frame)
    if event == 'call':
        caller = frame.f_back
        filename = caller.f_code.co_filename
        lineno = caller.f_lineno

        args_info = inspect.getargvalues(frame)
        args_types = {name: serialize(frame.f_locals.get(name))
                      for name in args_info.args}
        if args_info.varargs:
            args_types[args_info.varargs] = 'array'
        if args_info.keywords:
            args_types[args_info.keywords] = 'dict'

        if "self" in frame.f_locals:
            cls = type(frame.f_locals["self"]).__name__
            qual = f"{modname}.{cls}.{frame.f_code.co_name}"
        else:
            qual = f"{modname}.{frame.f_code.co_name}"

        record = {
            "func": qual,
            "file_dir": filename,
            "lineno": lineno,
            "args": args_types,
            "return": None
        }
        parts = record["func"].split(".")
        # parts e.g. ["toml","decoder","TomlDecoder","_load_line_multiline_str"]
        record["module"]    = parts[0]
        record["file_name"] = parts[1]
        record["func"]      = ".".join(parts[2:])  # "TomlDecoder._load_line_multiline_str"
        call_map[key] = record
        call_records.append(record)

    else:  # return
        record = call_map.pop(key, None)
        if record is not None:
            # 如果返回 tuple，就记录元素类型的 tuple
            if isinstance(arg, tuple):
                record["return"] = tuple(serialize(x) for x in arg)
            else:
                record["return"] = serialize(arg)
    return trace_calls

@atexit.register
def dump_trace():
    with open("trace.json", "w", encoding="utf-8") as f:
        json.dump(call_records, f, ensure_ascii=False, indent=2)

def trace2():
    sys.settrace(trace_calls)
    import toml
    data = toml.load("example-v0.4.0.toml")
    print(data)

def annotate_module_sources(source_pkg_dir: str, output_dir: str):
    """
    Walk source_pkg_dir for .py files, collect trace_records,
    shift multi-line call annotations to the closing-paren line,
    and append '# <var>: <Type>' comments, handling multiple return values.
    """
    # 1) 构建初始 mapping（abs-path -> orig_line -> set(types_or_tuples)）
    mapping = {}
    for rec in call_records:
        fn = rec.get("filename")
        ln = rec.get("lineno")
        if not fn or not ln:
            continue
        fn_norm = os.path.normpath(os.path.abspath(fn))

        if rec["func"].endswith(".__init__"):
            self_repr = rec["args"].get("self", "")
            m = re.match(r"<(\w+)", self_repr)
            t = m.group(1) if m else self_repr
        else:
            ret = rec.get("return")
            if isinstance(ret, tuple):
                t = ret
            elif isinstance(ret, str):
                t = ret
            else:
                args_types = rec.get("args", {}).values()
                t = tuple(sorted(set(args_types)))

        mapping.setdefault(fn_norm, {}).setdefault(ln, set()).add(t)

    # 2) 调整 mapping 到闭合行
    adjusted = {}
    for root, _, files in os.walk(source_pkg_dir):
        for name in files:
            if not name.endswith(".py"):
                continue
            src_path = os.path.join(root, name)
            src_norm = os.path.normpath(os.path.abspath(src_path))
            orig_map = mapping.get(src_norm, {})
            if not orig_map:
                continue

            lines = open(src_path, "r", encoding="utf-8").readlines()
            file_map = {}
            for orig_ln, types in orig_map.items():
                idx = orig_ln - 1
                balance = 0
                target_ln = orig_ln
                for j in range(idx, len(lines)):
                    balance += lines[j].count("(") - lines[j].count(")")
                    if balance <= 0:
                        target_ln = j + 1
                        break
                file_map.setdefault(target_ln, set()).update(types)
            adjusted[src_norm] = file_map

    # 3) 写入带注释的文件
    for root, _, files in os.walk(source_pkg_dir):
        for name in files:
            if not name.endswith(".py"):
                continue
            src_path = os.path.join(root, name)
            src_norm = os.path.normpath(os.path.abspath(src_path))
            rel = os.path.relpath(src_path, source_pkg_dir)
            dst_path = os.path.join(output_dir, rel)
            os.makedirs(os.path.dirname(dst_path), exist_ok=True)

            file_map = adjusted.get(src_norm, {})
            with open(src_path, "r", encoding="utf-8") as fin:
                lines = fin.readlines()

            with open(dst_path, "w", encoding="utf-8") as fout:
                for i, line in enumerate(lines, start=1):
                    clean = line.rstrip("\n")
                    types = file_map.get(i)
                    if not types:
                        fout.write(line)
                        continue

                    # 为每个 t 构建子注释字符串
                    comments = []
                    for t in types:
                        if isinstance(t, tuple):
                            comments.append(";    ".join(t))
                        else:
                            comments.append(t)
                    type_comment = " | ".join(comments)

                    # 尝试检测变量名或 return
                    stripped = clean.lstrip()
                    if stripped.startswith("return"):
                        label = "return"
                    else:
                        m = re.match(r"\s*([\w]+(?:\s*,\s*[\w]+)*)\s*=", clean)
                        if m:
                            vars = [v.strip() for v in m.group(1).split(",")]
                        else:
                            vars = []

                    # 生成最终注释
                    if len(vars) == 1:
                        comment = f"{vars[0]}: {type_comment}"
                    elif len(vars) > 1 and isinstance(next(iter(types)), tuple):
                        parts = [f"{var}: {typ}" for var, typ in zip(vars, next(iter(types)))]
                        comment = ";    ".join(parts)
                    else:
                        comment = type_comment

                    fout.write(f"{clean}  # {comment}\n")
    print(f"Annotated sources written to: {output_dir}")

if __name__ == "__main__":
    # 1) 运行 trace，收集类型信息
    trace2()
    # 2) 注释 toml 模块源码
    annotate_module_sources("toml", "annotated_toml_module")
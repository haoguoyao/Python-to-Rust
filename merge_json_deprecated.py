import json
from collections import defaultdict

def merge_json(static_ir_path, trace_path, output_path='merged.json'):
    # 1) 读取静态 IR
    with open(static_ir_path, 'r', encoding='utf-8') as f:
        ir_list = json.load(f)

    # 2) 读取动态 trace，按 func 收集所有记录
    with open(trace_path, 'r', encoding='utf-8') as f:
        trace_list = json.load(f)

    trace_map = defaultdict(list)
    for rec in trace_list:
        trace_map[rec['func']].append(rec)

    # 3) 对每个 func 的多条记录取 union
    aggregated = {}
    for func, recs in trace_map.items():
        args_union = defaultdict(set)
        returns_union = set()
        for rec in recs:
            # args: dict of name->type
            for name, typ in rec.get('args', {}).items():
                args_union[name].add(typ)
            # return: either a string or a list/tuple
            ret = rec.get('return')
            if isinstance(ret, (list, tuple)):
                returns_union.update(ret)
            elif ret is not None:
                returns_union.add(ret)
        # 转成列表或单一值
        agg_args = {name: list(types) for name, types in args_union.items()}
        agg_ret  = list(returns_union)
        aggregated[func] = {
            'args':   agg_args,
            'return': agg_ret if len(agg_ret)>1 else (agg_ret[0] if agg_ret else None)
        }

    # 4) 注入到 IR 并写出
    merged = []
    for ir in ir_list:
        fn = ir.get('qname')
        info = aggregated.get(fn)
        if info:
            print(f"合并动态类型: {fn} -> args: {info['args']}, return: {info['return']}")
            ir['args']   = info['args']
            ir['return'] = info['return']
        merged.append(ir)

    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(merged, f, ensure_ascii=False, indent=2)

    print(f"Merged JSON written to {output_path}")

if __name__ == '__main__':
    merge_json('project_ir.json', 'trace.json')
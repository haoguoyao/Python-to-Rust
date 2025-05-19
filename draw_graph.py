from networkx.drawing.nx_pydot import pydot_layout as graphviz_layout
import networkx as nx
import matplotlib.pyplot as plt
def draw_min_crossing(G):
    # 1) compute dot layout
    pos = graphviz_layout(G, prog='dot')
    labels = {n: n.name for n in G.nodes()}

    # 2) separate classes vs. functions
    clses = [n for n in G.nodes() if n.type == 'class']
    funcs = [n for n in G.nodes() if n.type == 'function']

    plt.figure(figsize=(12, 8))

    # 3) draw nodes by type
    nx.draw_networkx_nodes(
        G, pos,
        nodelist=clses,
        node_shape='s',
        node_color='lightblue',
        node_size=600,
        label='class'
    )
    nx.draw_networkx_nodes(
        G, pos,
        nodelist=funcs,
        node_shape='o',
        node_color='lightgreen',
        node_size=600,
        label='function'
    )

    # 4) draw edges
    nx.draw_networkx_edges(
        G, pos,
        arrows=True,
        edge_color='gray'
    )

    # 5) draw labels
    nx.draw_networkx_labels(
        G, pos,
        labels=labels,
        font_size=8
    )

    # 6) legend & finishing touches
    plt.legend(scatterpoints=1)
    plt.title("Dependency Graph")
    plt.axis('off')
    plt.tight_layout()
    plt.show()

def draw_original(G):
    pos = nx.spring_layout(G)
    labels = {n: n.name for n in G.nodes()}

    funcs = [n for n in G.nodes() if n.type=='function']
    clses = [n for n in G.nodes() if n.type=='class']

    # draw nodes without labels
    nx.draw_networkx_nodes(G, pos,
                           nodelist=clses, node_shape='s', node_color='lightblue',
                           label='class')
    nx.draw_networkx_nodes(G, pos,
                           nodelist=funcs, node_shape='o', node_color='lightgreen',
                           label='function')
    nx.draw_networkx_edges(G, pos)
    # now draw only our custom labels
    nx.draw_networkx_labels(G, pos, labels=labels, font_size=8)

    plt.legend(scatterpoints=1)
    plt.title("Dependency Graph")
    plt.axis('off')
    plt.show()
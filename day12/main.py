def read_input():
    inputs = {}
    with open('./input.txt') as f:
        for path in f:
            (n1, n2) = path.strip().split('-')
            if n1 in inputs:
                inputs[n1].append(n2)
            else:
                inputs[n1] = [n2]

            if n2 in inputs:
                inputs[n2].append(n1)
            else:
                inputs[n2] = [n1]
    inputs['end'] = []
    return inputs

def dfs(visited, graph, node, double=('', True)):
    paths = []
    if node not in visited:
        if len(graph[node]) == 0:
            return [[node]]
        if node.lower() == node:
            if double[1] == False and double[0] == node:
                double = (double[0], True)
            else:
                visited.add(node)
        for neighbour in graph[node]:
            paths += [[node]+path for path in dfs(visited.copy(), graph, neighbour, double)]
    return paths
        

def a(graph):
    visited = set()

    paths = dfs(visited, graph, 'start')
    return len(paths)

def b(graph):
    duplicatable_nodes = [key for key in graph.keys() if (key.lower() == key and key not in ['start','end'])]
    paths = []
    for duplicatable in duplicatable_nodes:
        visited = set()
        paths += dfs(visited, graph, 'start', (duplicatable, False))
    return len(set(map(tuple, paths)))
    

if __name__ == '__main__':
    graph = read_input()
    print(f"A: {a(graph)}")
    print(f"B: {b(graph)}")
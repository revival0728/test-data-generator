text = ''

with open('./test_file/test3.tdc') as f:
    text = f.read()


tmp = list(map(lambda x: x.strip().split(' ') ,text.strip().split('\n')))

res = []

for i in tmp:
    res.extend(list(map(lambda x: '"{}"'.format(x.replace('"', '\\"')), i)))

print(", ".join(res))

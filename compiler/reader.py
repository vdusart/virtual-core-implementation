f = open('../programs/file.bin', 'rb')
content = f.read()
for e in content:
    print(e)
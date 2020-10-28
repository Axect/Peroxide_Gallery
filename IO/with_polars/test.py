import pyarrow as pa

reader = pa.ipc.open_file("data/test.dat")
data = reader.read_pandas()

print(data)

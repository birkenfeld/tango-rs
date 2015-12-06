import PyTango
dev = PyTango.DeviceProxy('tango://localhost:10000/test/benchmark/echo')
for i in range(2000):
    print dev.Echo("This is a minimal Tango test client.")

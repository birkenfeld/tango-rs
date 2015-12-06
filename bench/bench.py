import PyTango
dev = PyTango.DeviceProxy('tango://localhost:10000/sys/tg_test/1')
for i in range(2000):
    print dev.DevString("This is a minimal Tango test client.")

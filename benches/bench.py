import time

import PyTango

dev = PyTango.DeviceProxy('tango://localhost:10000/sys/tg_test/1')
s = "This is a minimal Tango test client."

t1 = time.time()
for i in range(10000):
    assert dev.DevString(s) == s
t2 = time.time()
print 'per call: %d ns' % ((t2 - t1) * 1e5)

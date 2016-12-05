TANGO_ROOT = /usr

bench: bench.cpp
	g++ -O3 -std=c++0x -o bench bench.cpp -I $(TANGO_ROOT)/include -I $(TANGO_ROOT)/include/tango -L $(TANGO_ROOT)/lib -lomniORB4 -lomnithread -lomniDynamic4 -ltango 

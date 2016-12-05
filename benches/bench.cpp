#include <cstdio>
#include <chrono>
#include <tango.h>

int main() {
    Tango::DeviceProxy dev("tango://localhost:10000/sys/tg_test/1");
    std::chrono::time_point<std::chrono::system_clock> start, end;

    start = std::chrono::system_clock::now();
    for (int i=0; i < 10000; ++i) {
        Tango::DeviceData argin;
        std::string instr("This is a minimal Tango test client.");
        std::string outstr;
        argin << instr;
        Tango::DeviceData argout;
        try {
            argout = dev.command_inout("DevString", argin);
        } catch(Tango::DevFailed &fail) {
            std::cerr << fail.errors[0].desc << std::endl;
	    continue;
        }
        argout >> outstr;
	assert(outstr == instr);
    }
    end = std::chrono::system_clock::now();
    int elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(end-start).count() / 10000;
    std::cout << "per call: " << elapsed << " ns" << std::endl;
}

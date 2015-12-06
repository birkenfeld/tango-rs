#include <tango.h>
#include <stdio.h>

int main() {
    Tango::DeviceProxy dev("tango://localhost:10000/test/benchmark/echo");
    for (int i=0; i < 2000; ++i) {
        Tango::DeviceData argin;
        std::string instr("This is a minimal Tango test client.");
        std::string outstr;
        argin << instr;
        Tango::DeviceData argout;
        try {
            argout = dev.command_inout("Echo", argin);
        } catch(Tango::DevFailed &fail) {
            std::cerr << fail.errors[0].desc << std::endl;
	    continue;
        }
        argout >> outstr;
        std::cout << outstr << std::endl;
    }
}

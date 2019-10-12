binding:
	bindgen --ctypes-prefix libc --output src/c_tango/src/binding.rs src/c_tango/src/c_tango.h -- -DRUST_BINDGEN

# for testing

C_SOURCE=c_tango_attribute.c

onlyc:
	cd src/c_tango/src; g++ `pkg-config --cflags tango` -I. $(C_SOURCE)

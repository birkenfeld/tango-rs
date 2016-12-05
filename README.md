# tango-rs

This library is a client for [Tango control system](http://tango-controls.org)
servers.  It uses the preexisting C bindings (included) and wraps the API
functions provided by it in a Rustic interface.

Device proxy API calls:

* `command_inout`
* `command_query`
* `command_list_query`
* `get_attribute_list`
* `get_attribute_config`
* `attribute_list_query`
* `read/write_attribute`
* `read/write_attributes`
* `get/put/delete_device_property`
* `get/set_timeout`
* `get/set_source`
* `lock/unlock/is_locked`


Database API calls:

* `get_device_exported/_for_class`
* `get_object_list`
* `get_object_property_list`
* `get/put/delete_property`

## Building

You need an installed Tango library with headers.  Set the `PKG_CONFIG_PATH`
environment variable to the directory that contains `tango.pc` if it is not
found by default.

## Testing

For testing/benchmarking, you need a Tango database running on
`localhost:10000`, and the default instance of the standard testing server
(`TangoTest/test`).

Then, you can run `cargo test` to test all wrapped APIs, and `cargo bench` to
measure the duration of `command_inout` roundtrips.

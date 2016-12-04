# tango-rs

This library is a client for [Tango control system](http://tango-controls.org)
servers.  It uses the preexisting C bindings (included) and only wraps the
functions provided by it in a Rustic interface.

Covered major API calls:

* `command_inout`
* `command_query`
* `command_list_query`
* `get_attribute_list`
* `get_attribute_config`
* `attribute_list_query`
* `read_attribute`
* `write_attribute`
* `read_attributes`
* `write_attributes`

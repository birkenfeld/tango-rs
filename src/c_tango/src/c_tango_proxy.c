/******************************************************************************
 *
 * File:        c_tango_proxy.c
 * Project:     C/Rust client interface to Tango
 * Description: Interface functions to access Tango devices
 * Original:    November 2007
 * Author:      jensmeyer
 *
 * Adapted for tango-rs by Georg Brandl, 2015.
 *
 ******************************************************************************/

#include "c_tango.h"

ErrorStack *tango_translate_exception(Tango::DevFailed &tango_exception);


ErrorStack *tango_create_device_proxy(char *dev_name, void **proxy) {
    try {
        Tango::DeviceProxy *dev = new Tango::DeviceProxy(dev_name);
        *proxy = (void *)dev;
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_delete_device_proxy(void *proxy) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        delete dev;
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_set_timeout_millis(void *proxy, int millis) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        dev->set_timeout_millis(millis);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_timeout_millis(void *proxy, int *millis) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        *millis = dev->get_timeout_millis();
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

ErrorStack *tango_set_source(void *proxy, DevSource source) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        dev->set_source((Tango::DevSource)source);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

ErrorStack *tango_get_source(void *proxy, DevSource *source) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        *source = (DevSource)dev->get_source();
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_lock (void *proxy) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        dev->lock();
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

ErrorStack *tango_unlock (void *proxy) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        dev->unlock();
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

ErrorStack *tango_is_locked (void *proxy, bool *is_locked) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        *is_locked = dev->is_locked();
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

ErrorStack *tango_is_locked_by_me(void *proxy, bool *is_locked_by_me) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        *is_locked_by_me = dev->is_locked_by_me();
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

ErrorStack *tango_locking_status(void *proxy, char **locking_status) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;

    try {
        string st = dev->locking_status();
        *locking_status = strdup(st.c_str());
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


void tango_free_ErrorStack(ErrorStack *error) {
    for (uint32_t i = 0; i < error->length; i++) {
        free(error->sequence[i].desc);  // from strdup
        free(error->sequence[i].reason);
        free(error->sequence[i].origin);
    }
    delete[] error->sequence;
    delete error;
}


ErrorStack *tango_translate_exception(Tango::DevFailed &tango_exception) {
    ErrorStack *error = new ErrorStack;
    INIT_SEQ(*error, DevFailed, tango_exception.errors.length());

    for (uint32_t i = 0; i < tango_exception.errors.length(); i++) {
        error->sequence[i].desc = strdup(tango_exception.errors[i].desc);
        error->sequence[i].reason = strdup(tango_exception.errors[i].reason);
        error->sequence[i].origin = strdup(tango_exception.errors[i].origin);
        error->sequence[i].severity = (ErrSeverity)tango_exception.errors[i].severity;
    }

    return error;
}

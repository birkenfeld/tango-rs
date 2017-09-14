/******************************************************************************
 *
 * File:        c_tango_command.c
 * Project:     C client interface to Tango
 * Description: Interface functions to access Tango commands
 * Original:    November 2007
 * Author:      jensmeyer
 *
 * Adapted for tango-rs by Georg Brandl, 2015.
 *
 ******************************************************************************/

#include "c_tango.h"
#include <tango.h>

ErrorStack *tango_translate_exception(Tango::DevFailed& tango_exception);
static void convert_cmd_query(Tango::CommandInfo& tango_cmd_info, CommandInfo *cmd_info);


ErrorStack *tango_command_inout(void *proxy, char *cmd_name, CommandData *argin, CommandData *argout) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::DeviceData cmd_in;
    Tango::DeviceData cmd_out;

    try {
        // convert the input argument
        switch (argin->arg_type) {
        case DEV_VOID:
            break;

        case DEV_BOOLEAN:
            cmd_in << argin->cmd_data.bool_val;
            break;

        case DEV_SHORT:
            cmd_in << argin->cmd_data.short_val;
            break;

        case DEV_USHORT:
            cmd_in << argin->cmd_data.ushort_val;
            break;

        case DEV_LONG:
            cmd_in << (Tango::DevLong)argin->cmd_data.long_val;
            break;

        case DEV_ULONG:
            cmd_in << (Tango::DevULong)argin->cmd_data.ulong_val;
            break;

        case DEV_LONG64:
            cmd_in << (Tango::DevLong64)argin->cmd_data.long64_val;
            break;

        case DEV_ULONG64:
            cmd_in << (Tango::DevULong64)argin->cmd_data.ulong64_val;
            break;

        case DEV_FLOAT:
            cmd_in << argin->cmd_data.float_val;
            break;

        case DEV_DOUBLE:
            cmd_in << argin->cmd_data.double_val;
            break;

        case DEV_STRING:
        case CONST_DEV_STRING:
            cmd_in << argin->cmd_data.string_val;
            break;

        case DEV_ENCODED: {
            Tango::DevVarCharArray tmp(argin->cmd_data.encoded_val.encoded_length,
                                       argin->cmd_data.encoded_val.encoded_length,
                                       argin->cmd_data.encoded_val.encoded_data);
            cmd_in.insert(argin->cmd_data.encoded_val.encoded_format, &tmp);
            break;
        }

#define INSERT_ARRAY(member, type)                                      \
            {                                                           \
                vector<type> arr(argin->cmd_data.member.length);        \
                memcpy(arr.data(), argin->cmd_data.char_arr.sequence,   \
                       sizeof(type) * argin->cmd_data.char_arr.length); \
                cmd_in << arr;                                          \
                break;                                                  \
            }
            
        case DEVVAR_CHARARRAY:    INSERT_ARRAY(char_arr, uint8_t);
        case DEVVAR_SHORTARRAY:   INSERT_ARRAY(short_arr, int16_t);
        case DEVVAR_USHORTARRAY:  INSERT_ARRAY(ushort_arr, uint16_t);
        case DEVVAR_LONGARRAY:    INSERT_ARRAY(long_arr, int32_t);
        case DEVVAR_ULONGARRAY:   INSERT_ARRAY(ulong_arr, uint32_t);
        case DEVVAR_LONG64ARRAY:  INSERT_ARRAY(long64_arr, int64_t);
        case DEVVAR_ULONG64ARRAY: INSERT_ARRAY(ulong64_arr, uint64_t);
        case DEVVAR_FLOATARRAY:   INSERT_ARRAY(float_arr, float);
        case DEVVAR_DOUBLEARRAY:  INSERT_ARRAY(double_arr, double);

        case DEVVAR_STRINGARRAY: {
            vector<string> string_arr(argin->cmd_data.string_arr.length);

            for (uint32_t i=0; i<argin->cmd_data.string_arr.length; i++) {
                string_arr[i] = argin->cmd_data.string_arr.sequence[i];
            }

            cmd_in << string_arr;
            break;
        }

        case DEVVAR_LONGSTRINGARRAY: {
            vector<Tango::DevLong> long_arr(argin->cmd_data.long_string_arr.long_length);
            vector<string> string_arr(argin->cmd_data.long_string_arr.string_length);

            for (uint32_t i = 0; i < argin->cmd_data.long_string_arr.long_length; i++) {
                long_arr[i] = argin->cmd_data.long_string_arr.long_sequence[i];
            }
            for (uint32_t i = 0; i < argin->cmd_data.long_string_arr.string_length; i++) {
                string_arr[i] = argin->cmd_data.long_string_arr.string_sequence[i];
            }

            cmd_in.insert(long_arr, string_arr);
            break;
        }

        case DEVVAR_DOUBLESTRINGARRAY: {
            vector<double> double_arr(argin->cmd_data.double_string_arr.double_length);
            vector<string> string_arr(argin->cmd_data.double_string_arr.string_length);

            for (uint32_t i = 0; i < argin->cmd_data.double_string_arr.double_length; i++) {
                double_arr[i] = argin->cmd_data.double_string_arr.double_sequence[i];
            }
            for (uint32_t i = 0; i < argin->cmd_data.double_string_arr.string_length; i++) {
                string_arr[i] = argin->cmd_data.double_string_arr.string_sequence[i];
            }

            cmd_in.insert(double_arr, string_arr);
            break;
        }

        default:
            Tango::Except::throw_exception(
                "Data type error",
                "The requested data type is not implemented for command writing!",
                "c_tango_command.c::tango_command_inout()");
            break;
        }

        // treat the void case!
        if (argin->arg_type == DEV_VOID) {
            cmd_out = dev->command_inout(cmd_name);
        } else {
            cmd_out = dev->command_inout(cmd_name, cmd_in);
        }
        try {
            cmd_out.is_empty();
            argout->arg_type = (TangoDataType)cmd_out.get_type();
        } catch (Tango::DevFailed &e) {
            argout->arg_type = DEV_VOID;
        }

        // convert the output argument
        switch (argout->arg_type) {
        case DEV_VOID:
            break;

        case DEV_BOOLEAN:
            cmd_out >> argout->cmd_data.bool_val;
            break;

        case DEV_SHORT:
            cmd_out >> argout->cmd_data.short_val;
            break;

        case DEV_USHORT:
            cmd_out >> argout->cmd_data.ushort_val;
            break;

        case DEV_LONG: {
            Tango::DevLong long_val;
            cmd_out >> long_val;
            argout->cmd_data.long_val = long_val;
            break;
        }

        case DEV_ULONG: {
            Tango::DevULong ulong_val;
            cmd_out >> ulong_val;
            argout->cmd_data.ulong_val = ulong_val;
            break;
        }

        case DEV_LONG64: {
            Tango::DevLong64 long64_val;
            cmd_out >> long64_val;
            argout->cmd_data.long64_val = long64_val;
            break;
        }

        case DEV_ULONG64: {
            Tango::DevULong64 ulong64_val;
            cmd_out >> ulong64_val;
            argout->cmd_data.ulong64_val = ulong64_val;
            break;
        }

        case DEV_FLOAT:
            cmd_out >> argout->cmd_data.float_val;
            break;

        case DEV_DOUBLE:
            cmd_out >> argout->cmd_data.double_val;
            break;

        case DEV_STATE: {
            Tango::DevState state_val;
            cmd_out >> state_val;
            argout->cmd_data.state_val = (TangoDevState)state_val;
            break;
        }

        case DEV_STRING:
        case CONST_DEV_STRING: {
            string string_val;
            cmd_out >> string_val;
            argout->cmd_data.string_val = strdup(string_val.c_str());
            break;
        }

        case DEV_ENCODED: {
            Tango::DevEncoded encoded_val;
            cmd_out >> encoded_val;

            string format(encoded_val.encoded_format);
            argout->cmd_data.encoded_val.encoded_format = strdup(format.c_str());

            // get the pointer to the buffer and take over the memory ("true")
            argout->cmd_data.encoded_val.encoded_length =
                encoded_val.encoded_data.length();
            argout->cmd_data.encoded_val.encoded_data =
                (unsigned char *)encoded_val.encoded_data.get_buffer(true);
            break;
        }

#define EXTRACT_ARRAY(member, tgtype)                                   \
            {                                                           \
                const Tango::tgtype *seq;                               \
                cmd_out >> seq;                                         \
                argout->cmd_data.member.length = seq->length();         \
                argout->cmd_data.member.sequence =                      \
                    ((Tango::tgtype *)seq)->get_buffer(true);           \
                break;                                                  \
            }

        case DEVVAR_CHARARRAY:    EXTRACT_ARRAY(char_arr, DevVarCharArray);
        case DEVVAR_SHORTARRAY:   EXTRACT_ARRAY(short_arr, DevVarShortArray);
        case DEVVAR_USHORTARRAY:  EXTRACT_ARRAY(ushort_arr, DevVarUShortArray);
        case DEVVAR_LONGARRAY:    EXTRACT_ARRAY(long_arr, DevVarLongArray);
        case DEVVAR_ULONGARRAY:   EXTRACT_ARRAY(ulong_arr, DevVarULongArray);
        case DEVVAR_LONG64ARRAY:  EXTRACT_ARRAY(long64_arr, DevVarLong64Array);
        case DEVVAR_ULONG64ARRAY: EXTRACT_ARRAY(ulong64_arr, DevVarULong64Array);
        case DEVVAR_FLOATARRAY:   EXTRACT_ARRAY(float_arr, DevVarFloatArray);
        case DEVVAR_DOUBLEARRAY:  EXTRACT_ARRAY(double_arr, DevVarDoubleArray);

        case DEVVAR_STRINGARRAY: {
            vector<string> string_vect;
            int nb_data;
            
            cmd_out >> string_vect;
            nb_data = string_vect.size();

            argout->cmd_data.string_arr.sequence = new char *[nb_data];
            argout->cmd_data.string_arr.length = nb_data;

            for (int i = 0; i < nb_data; i++) {
                argout->cmd_data.string_arr.sequence[i] = strdup(string_vect[i].c_str());
            }
            break;
        }

        case DEVVAR_LONGSTRINGARRAY: {
            vector<Tango::DevLong> long_vect;
            vector<string> string_vect;
            int nb_data;

            cmd_out.extract(long_vect, string_vect);

            nb_data = long_vect.size();
            argout->cmd_data.long_string_arr.long_sequence = new int[nb_data];
            argout->cmd_data.long_string_arr.long_length = nb_data;
            for (int i = 0; i < nb_data; i++) {
                argout->cmd_data.long_string_arr.long_sequence[i] = long_vect[i];
            }

            nb_data = string_vect.size();
            argout->cmd_data.long_string_arr.string_sequence = new char *[nb_data];
            argout->cmd_data.long_string_arr.string_length = nb_data;
            for (int i = 0; i < nb_data; i++) {
                argout->cmd_data.long_string_arr.string_sequence[i] =
                    strdup(string_vect[i].c_str());
            }
            break;
        }
            
        case DEVVAR_DOUBLESTRINGARRAY: {
            vector<double> double_vect;
            vector<string> string_vect;
            int nb_data;

            cmd_out.extract (double_vect, string_vect);

            nb_data = double_vect.size();
            argout->cmd_data.double_string_arr.double_sequence = new double[nb_data];
            argout->cmd_data.double_string_arr.double_length = nb_data;
            for (int i = 0; i < nb_data; i++) {
                argout->cmd_data.double_string_arr.double_sequence[i] = double_vect[i];
            }

            nb_data = string_vect.size();
            argout->cmd_data.double_string_arr.string_sequence = new char *[nb_data];
            argout->cmd_data.double_string_arr.string_length = nb_data;
            for (int i = 0; i < nb_data; i++) {
                argout->cmd_data.double_string_arr.string_sequence[i] =
                    strdup(string_vect[i].c_str());
            }
            break;
        }
            
        default:
            Tango::Except::throw_exception(
                "Data type error",
                "The requested data type is not implemented for command reading!",
                "c_tango_command.c::tango_command_inout()");
            break;
        }
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


void tango_free_CommandData(CommandData *command_data) {
    switch (command_data->arg_type) {
    case DEV_STRING:
        free(command_data->cmd_data.string_val);  // from strdup
        break;

    case DEV_ENCODED:
        free(command_data->cmd_data.encoded_val.encoded_format);  // from strdup
        delete[] command_data->cmd_data.encoded_val.encoded_data;
        break;

    case DEVVAR_CHARARRAY:
        delete[] command_data->cmd_data.char_arr.sequence;
        break;

    case DEVVAR_SHORTARRAY:
        delete[] command_data->cmd_data.short_arr.sequence;
        break;

    case DEVVAR_USHORTARRAY:
        delete[] command_data->cmd_data.ushort_arr.sequence;
        break;

    case DEVVAR_LONGARRAY:
        delete[] command_data->cmd_data.long_arr.sequence;
        break;

    case DEVVAR_ULONGARRAY:
        delete[] command_data->cmd_data.ulong_arr.sequence;
        break;

    case DEVVAR_LONG64ARRAY:
        delete[] command_data->cmd_data.long64_arr.sequence;
        break;

    case DEVVAR_ULONG64ARRAY:
        delete[] command_data->cmd_data.ulong64_arr.sequence;
        break;

    case DEVVAR_FLOATARRAY:
        delete[] command_data->cmd_data.float_arr.sequence;
        break;

    case DEVVAR_DOUBLEARRAY:
        delete[] command_data->cmd_data.double_arr.sequence;
        break;
        
    case DEVVAR_STRINGARRAY:
        for (uint32_t i = 0; i < command_data->cmd_data.string_arr.length; i++) {
            free(command_data->cmd_data.string_arr.sequence[i]);  // from strdup
        }
        delete[] command_data->cmd_data.string_arr.sequence;
        break;

    case DEVVAR_LONGSTRINGARRAY:
        delete[] command_data->cmd_data.long_string_arr.long_sequence;
        for (uint32_t i = 0; i < command_data->cmd_data.long_string_arr.string_length; i++) {
            free(command_data->cmd_data.long_string_arr.string_sequence[i]);  // from strdup
        }
        delete[] command_data->cmd_data.long_string_arr.string_sequence;
        break;

    case DEVVAR_DOUBLESTRINGARRAY:
        delete[] command_data->cmd_data.double_string_arr.double_sequence;
        for (uint32_t i = 0; i<command_data->cmd_data.double_string_arr.string_length; i++) {
            free(command_data->cmd_data.double_string_arr.string_sequence[i]);  // from strdup
        }
        delete[] command_data->cmd_data.double_string_arr.string_sequence;
        break;

    default:
        break;
    }
}


ErrorStack *tango_command_query(void *proxy, char *cmd_name, CommandInfo *cmd_info) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::CommandInfo tango_cmd_info;

    try {
        tango_cmd_info = dev->command_query(cmd_name);
        convert_cmd_query(tango_cmd_info, cmd_info);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_command_list_query(void *proxy, CommandInfoList *cmd_info_list) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    vector<Tango::CommandInfo> *tango_cmd_info_list = 0;

    try {
        tango_cmd_info_list = dev->command_list_query();

        INIT_SEQ(*cmd_info_list, CommandInfo, tango_cmd_info_list->size());

        for (uint32_t i = 0; i < tango_cmd_info_list->size(); i++) {
            convert_cmd_query((*tango_cmd_info_list)[i], &cmd_info_list->sequence[i]);
        }
        delete tango_cmd_info_list;
    }
    catch (Tango::DevFailed &tango_exception) {
        if (tango_cmd_info_list)
            delete tango_cmd_info_list;
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


void tango_free_CommandInfo(CommandInfo *command_info) {
    free(command_info->cmd_name);  // from strdup
    free(command_info->in_type_desc);
    free(command_info->out_type_desc);
}


void tango_free_CommandInfoList(CommandInfoList *command_info_list) {
    for (uint32_t i = 0; i < command_info_list->length; i++) {
        tango_free_CommandInfo(&(command_info_list->sequence[i]));
    }

    delete[] command_info_list->sequence;
}


static void convert_cmd_query(Tango::CommandInfo& tango_cmd_info, CommandInfo *cmd_info) {
    cmd_info->cmd_name = strdup(tango_cmd_info.cmd_name.c_str());
    cmd_info->in_type_desc = strdup(tango_cmd_info.in_type_desc.c_str());
    cmd_info->out_type_desc = strdup(tango_cmd_info.out_type_desc.c_str());

    cmd_info->cmd_tag = tango_cmd_info.cmd_tag;
    cmd_info->in_type = tango_cmd_info.in_type;
    cmd_info->out_type = tango_cmd_info.out_type;
    cmd_info->disp_level = (DispLevel)tango_cmd_info.disp_level;
}

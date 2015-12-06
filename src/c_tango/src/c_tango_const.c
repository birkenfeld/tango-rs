/******************************************************************************
 * 
 * File       :	c_tango_const.c
 *
 * Project    :	C client interface to Tango
 * 
 * Description:	Static data definitions to write Tango clients in C
 *
 * Original   :	November 2007	
 *	
 * $Author: jensmeyer $
 *
 * $Log$
 * Revision 1.1  2007/12/06 08:01:25  jensmeyer
 * Created c_tango_const.c for static string arrays
 *
 *
 *
 ******************************************************************************/ 
/*
 * a globally defined dictionary for known data types
 */

const char *TangoDataTypeName[] = {
	"DevVoid",
	"DevBoolean",
	"DevShort",
	"DevLong",
	"DevFloat",
	"DevDouble",
	"DevUShort",
	"DevULong",
	"DevString",
	"DevVarCharArray",
	"DevVarShortArray",
	"DevVarLongArray",
	"DevVarFloatArray",
	"DevVarDoubleArray",
	"DevVarUShortArray",
	"DevVarULongArray",
	"DevVarStringArray",
	"DevVarLongStringArray",
	"DevVarDoubleStringArray",
	"DevState",
	"ConstDevString",
	"DevVarBooleanArray",
	"DevUChar",
	"DevLong64",
	"DevULong64",
	"DevVarLong64Array",
	"DevVarULong64Array",
	"DevInt"
};

/*
 * a globally defined dictionary for known device states
 */
 
const char *TangoDevStateName[] = {
	"ON",
	"OFF",
	"CLOSE",
	"OPEN",
	"INSERT",
	"EXTRACT",
	"MOVING",
	"STANDBY",
	"FAULT",
	"INIT",
	"RUNNING",
	"ALARM",
	"DISABLE",
	"UNKNOWN"
};

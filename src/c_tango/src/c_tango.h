/******************************************************************************
 *
 * File       :	c_tango.h
 *
 * Project    :	C client interface to Tango
 * 
 * Description:	Definitions necessay to write Tango clients in C
 *
 * Original   :	November 2007	
 *	
 * $Author: jensmeyer $
 *
 * $Log: c_tango.h,v $
 * Revision 1.8  2011/01/26 12:04:57  jensmeyer
 * Added a "Bricolage" to allow a command ReadImage with an encoded return type. This command is mapped to two attributes.
 * This should stay only until Tango allows commands with encoded data types.
 *
 * Revision 1.7  2010/12/17 14:25:01  jensmeyer
 * Added support for attributes with DevEncoded type and added methods
 * for device locking.
 *
 * Revision 1.6  2007/12/20 07:57:02  jensmeyer   
 * Corrected file headers
 *
 * Revision 1.5  2007/12/18 17:26:20  jensmeyer
 * Added new file for database access and corrected bugs
 *
 * Revision 1.4  2007/12/12 14:20:50  jensmeyer
 * Added doxygen documentation headers and commented code
 *
 * Revision 1.3  2007/12/07 16:05:15  jensmeyer
 * Some name changes to be comaptible with Taco
 *
 * Revision 1.2  2007/12/06 08:01:25  jensmeyer
 * Created c_tango_const.c for static string arrays
 *
 * Revision 1.1.1.1  2007/12/05 15:05:04  jensmeyer
 * Tango C language binding
 *
 ******************************************************************************/ 

 /**
 * \mainpage The Tango C Language Binding
 *
 * The Tango C language binding is a reduced C interface which 
 * wraps the Tango C++ API. The actual binding only contains the
 * basic query functionality and the basic synchronous reading and 
 * writing of commands and attributes.
 *
 * The API is structured in proxy related functions, command related 
 * functions and attribute related functions.
 *
 * \subpage Proxy
 *
 * \subpage Command
 *
 * \subpage Attribute
 *
 * \subpage Property
 *
 * \subpage Error
 *
 * \subpage Enum
 *
 * \subpage Type
 *
 * \subpage Struct
 */
 
#ifndef	C_TANGO_H
#define	C_TANGO_H


#include <sys/time.h>

#ifndef __cplusplus
#include <stdbool.h>
#endif


/* Remapping of Tango enumerations.
   This is necessary because in C we don`t have namespaces */

/**
 * \defgroup Enum Tango C Binding Enumerations
 *
 * All enumerations used in the Tango C binding.
 */
/*@{*/
/**
 * \enum TangoDataType
 * \brief All available Tango data types.
 *
 * The list of all available Tango data types. 
 * Scalar types and array types.
 */
enum TangoDataType {
	DEV_VOID = 0, 					/**<void  */
	DEV_BOOLEAN,					/**<bool  */
	DEV_SHORT,						/**<short  */
	DEV_LONG,						/**<int (32bits)  */
	DEV_FLOAT,						/**<float  */
	DEV_DOUBLE,						/**<double  */
	DEV_USHORT,						/**<unsigned short  */
	DEV_ULONG,						/**<unsigned long  */
	DEV_STRING,						/**<char *  */
	DEVVAR_CHARARRAY,				/**<array of unsigned char  */
	DEVVAR_SHORTARRAY,				/**<array of short */
	DEVVAR_LONGARRAY,				/**<array of int (32bits) */
	DEVVAR_FLOATARRAY,				/**<array of float */
	DEVVAR_DOUBLEARRAY,				/**<array of double */
	DEVVAR_USHORTARRAY,				/**<array of unsigned short */
	DEVVAR_ULONGARRAY,				/**<array of unsigned int (32bits) */
	DEVVAR_STRINGARRAY,				/**<array of char * */
	DEVVAR_LONGSTRINGARRAY,			/**<array of unsigned int (32bits) followed by an array of char * **/
	DEVVAR_DOUBLESTRINGARRAY,		/**<array of double followed by an array of char * */
	DEV_STATE,						/**<TangoDevState enumeration */
	CONST_DEV_STRING,				/**<const char * */
	DEVVAR_BOOLEANARRAY,			/**<array of bool */
	DEV_UCHAR,						/**<unsigned char  */
	DEV_LONG64,						/**<long or long long (64bits)  */
	DEV_ULONG64,					/**<unsigned long or unsigned long long (64bits)  */
	DEVVAR_LONG64ARRAY,				/**<array of long or long long (64bits)  */
	DEVVAR_ULONG64ARRAY,			/**<array of unsigned long or unsigned long long (64bits)  */
	DEV_INT,						/**<int (32bits)  */
	DEV_ENCODED						/**<Endoed data, description + buffer */
};


/* Tango defice state definitions */

/**
 * \enum TangoDevState
 * \brief The Tango Device States.
 *
 * The list of all possible states for Tango devices.
 * Every state is reperesented with a fixed color on 
 * the application level.
 */
enum TangoDevState { 
	ON, 			/**< The device is switched ON (green). */ 
	OFF, 			/**< The device is switched OFF (white). */
	CLOSE, 		/**< The device is CLOSED (white). */ 
	OPEN, 		/**< The device is OPEN (green). */ 
	INSERT, 		/**< The device is INSERTED to the beam (white). */ 
	EXTRACT, 	/**< The device is EXTRACTED from the beam (green). */ 
	MOVING, 		/**< The device is MOVING or in a state transition (blue). */ 
	STANDBY, 	/**< The device is STANDBY (yellow). */ 
	FAULT, 		/**< The device has detected a FAULT (red). */ 
	INIT, 		/**< The device is INITialising (beige). */ 
	RUNNING, 	/**< The device is RUNNING and doing some work (blue) */ 
	ALARM, 		/**< The device indicates an ALARM (orange). */ 
	DISABLE, 	/**< The device is DISABLED by an interlock (magenta). */ 
	UNKNOWN 		/**< The device lost its connection, the state is UNKNOWN (gray). */ 
};


/* Attribute releated definitions */

/**
 * \enum AttrQuality
 * \brief The attribute quality factor.
 *
 * The list of all possible attribute data quality factors.
 * Every read attribute data has an assigned quality value to indicate
 * the data validity.
 */
enum AttrQuality { 
	ATTR_VALID, 	/**< The attribute data is VALID. */ 
	ATTR_INVALID,  /**< The attribute data is INVALID. */ 
	ATTR_ALARM, 	/**< The attribute indicates an ALARM on the data. */ 
	ATTR_CHANGING, /**< The attribute value is CHANGING and not stable. */ 
	ATTR_WARNING   /**< The attribute indicates an WARNING on the data. */
};

/**
 * \enum AttrWriteType
 * \brief The attribute write type.
 *
 * The list of all possible attribute types.
 * An attribute can be read only, write only or read/write.
 */
enum AttrWriteType { 
	READ, 				/**< The attribute is read only. */
	READ_WITH_WRITE,  /**< The attribute is of type read with a second write attribute associated. */
	WRITE, 				/**< The attribute is write only. */
	READ_WRITE			/**< The attribute is of type read/write. */
};

/**
 * \enum AttrDataFormat
 * \brief The attribute data format.
 *
 * The data format of an attribute can be a scalar attribute, a spectrum 
 * (1D array) attribute or an image (2D array) attribute.
 */
enum AttrDataFormat { 
	SCALAR, 		/**< The attribute handles scalar values. */
	SPECTRUM, 	/**< The attribute handles a spectrum (1D array). */
	IMAGE			/**< The attribute handles an image (2D array). */
};

/**
 * \enum DispLevel
 * \brief The attribute display level.
 *
 * The attribute might be displayed for expert users only.
 */
enum DispLevel { 
	OPERATOR, 	/**< Attribute display all users. */
	EXPERT		/**< Attribute display only for expert users. */
};


/* Error related definitions */

/**
 * \enum ErrSeverity
 * \brief The error severity level.
 *
 * A Tango error can indicate three different severity levels.
 */
enum ErrSeverity { 
	WARN, 		/**< Warning level */
	ERR, 			/**< Error level */
	PANIC			/**< Real severe Panic level */
};

/* Proxy releated definitions */

/**
 * \enum DevSource
 * \brief The Tango data source.
 *
 * Data can be read directly from the device or from the polling cache.
 * In the case of CACHE_DEV, the data is read from the cache when it is 
 * available, otherwise from the device. This is the default setting.
 */
enum DevSource { 
	DEV, 				/**< Direct device reading */	
	CACHE, 			/**< Reading only from polling cache */
	CACHE_DEV		/**< Reading from chache or device */
};
/*@}*/


#ifndef __cplusplus
typedef enum TangoDevState TangoDevState;
typedef enum TangoDataType TangoDataType;
typedef enum AttrQuality AttrQuality;
typedef enum AttrWriteType AttrWriteType;
typedef enum AttrDataFormat AttrDataFormat;
typedef enum DispLevel DispLevel;
typedef enum ErrSeverity ErrSeverity;
typedef enum DevSource DevSource;
#endif


/**
 * \defgroup Type Tango Data Type Related Definitions
 *
 *  Tango data type definitions for the encoded data type,
 *  array data types and for long data types.
 *  The long data types should be used
 *  to avoid the 32/64 bit problem.
 */
/*@{*/

/* Handle the long proplem for 64 bit architectures */

/** \typedef TangoDevLong
 * A 32 bit long value
 */
typedef int 				TangoDevLong;
/** \typedef TangoDevULong
 * A 32 bit unsigned long value
 */
typedef unsigned int 		TangoDevULong;

/** \typedef TangoDevLong64
 * A 64 bit long value
 */
/** \typedef TangoDevULong64
 * A 64 bit unsigned long value
 */
#if __WORDSIZE == 64
typedef long             TangoDevLong64;
typedef unsigned long      TangoDevULong64;
#else
typedef long long         TangoDevLong64;
typedef unsigned long long  TangoDevULong64;
#endif

/* Structure for the encoded data type */

/**
 * \struct TangoDevEncoded
 * A structure containing a data description string and a pointer to the data buffer.
 */
struct TangoDevEncoded	{
							char 		  *encoded_format;
							unsigned int   encoded_length;
							unsigned char *encoded_data;
							};
typedef struct  TangoDevEncoded	TangoDevEncoded;

/* Array data structures */

/**
 * \struct VarBoolArray 
 * A structure containing a pointer to a sequence of boolean values and the number of elements in the sequence.
 */
struct VarBoolArray	{
							unsigned int length;
							bool   *sequence;
							};
typedef struct VarBoolArray 	VarBoolArray;
/**
 * \struct VarCharArray 
 * A structure containing a pointer to a sequence of char values and the number of elements in the sequence.
 */
struct VarCharArray	{
							unsigned int length;
							unsigned char *sequence;
							};
typedef struct VarCharArray 	VarCharArray;
/**
 * \struct VarShortArray
 * A structure containing a pointer to a sequence of short values and the number of elements in the sequence.
 */
struct VarShortArray	{
							unsigned int length;
							short  	  *sequence;
							};
typedef struct VarShortArray 	VarShortArray;
/**
 * \struct VarUShortArray
 * A structure containing a pointer to a sequence of unsigned short values and the number of elements in the sequence.
 */
struct VarUShortArray	{
							unsigned int 	length;
							unsigned short  *sequence;
							};
typedef struct VarUShortArray 	VarUShortArray;

/* Structures for 32 bit long values */

/**
 * \struct VarLongArray
 * A structure containing a pointer to a sequence of 32 bit long values and the number of elements in the sequence.
 */
struct VarLongArray	{
							unsigned int 		length;
							TangoDevLong 	*sequence;
							};
typedef struct VarLongArray 	VarLongArray;
/**
 * \struct VarULongArray
 * A structure containing a pointer to a sequence of 32 bit unsigned long values and the number of elements in the sequence.
 */
struct VarULongArray	{
							unsigned int 		length;
							TangoDevULong	*sequence;
							};
typedef struct VarULongArray 	VarULongArray;


/* Structures for 64 bit long values */

/**
 * \struct VarLong64Array
 * A structure containing a pointer to a sequence of 64 bit long values and the number of elements in the sequence.
 */
struct VarLong64Array	{
							unsigned int 		length;
							TangoDevLong64 *sequence;
							};
typedef struct VarLong64Array 	VarLong64Array;
/**
 * \struct VarULong64Array
 * A structure containing a pointer to a sequence of 64 bit unsigned long values 
 * and the number of elements in the sequence.
 */
struct VarULong64Array	{
							unsigned int  			length;
							TangoDevULong64 	*sequence;
							};
typedef struct VarULong64Array 	VarULong64Array;

/**
 * \struct VarFloatArray
 * A structure containing a pointer to a sequence of float values and the number of elements in the sequence.
 */
struct VarFloatArray {
							unsigned int length;
							float  	  *sequence;
							};								
typedef struct VarFloatArray VarFloatArray;
/**
 * \struct VarDoubleArray
 * A structure containing a pointer to a sequence of double values and the number of elements in the sequence.
 */
struct VarDoubleArray {
							unsigned int length;
							double  		*sequence;
							};								
typedef struct VarDoubleArray VarDoubleArray;
/**
 * \struct VarStringArray
 * A structure containing a pointer to a sequence of strings and the number of elements in the sequence.
 */
struct VarStringArray {
							unsigned int length;
							char  	  **sequence;
							};								
typedef struct VarStringArray VarStringArray;
/**
 * \struct VarStateArray
 * A structure containing a pointer to a sequence of  TangoDevState values 
 * and the number of elements in the sequence.
 */
struct VarStateArray	{
							unsigned int length;
							TangoDevState  *sequence;
							};								
typedef struct VarStateArray VarStateArray;
/**
 * \struct VarEncodedArray
 * A structure containing a pointer to a sequence of  TangoDevEncoded values 
 * and the number of elements in the sequence.
 */
struct VarEncodedArray	{
							unsigned int      length;
							TangoDevEncoded  *sequence;
							};								
typedef struct VarEncodedArray VarEncodedArray;
/**
 * \struct VarLongStringArray
 * A structure containing a pointer to a sequence of long and the number of elements in the sequence
 * as well as a pointer to a sequence of strings and the number of elements in the sequence.
 */
struct VarLongStringArray {
							unsigned int  long_length;
							TangoDevLong *long_sequence;
							unsigned int  string_length;
							char  	    **string_sequence;
							};								
typedef struct VarLongStringArray VarLongStringArray;
/**
 * \struct VarDoubleStringArray
 * A structure containing a pointer to a sequence of double and the number of elements in the sequence
 * as well as a pointer to a sequence of strings and the number of elements in the sequence.
 */
struct VarDoubleStringArray {
							unsigned int   double_length;
							double  	  *double_sequence;
							unsigned int   string_length;
							char  	     **string_sequence;
							};								
typedef struct VarDoubleStringArray VarDoubleStringArray;
/*@}*/



/**
 * \defgroup Struct Tango C Binding Data Structures
 *
 *  Data structures used in the Tango C binding.
 */
/*@{*/

/* union type to replace the CORBA ANY type */

/**
 * \union TangoAttributeData
 * An union of all Tango array data types used for attribute reading and writing.
 */
union TangoAttributeData {
				VarBoolArray   bool_arr;
				VarCharArray	char_arr;
				VarShortArray	short_arr;
				VarUShortArray	ushort_arr;
				VarLongArray	long_arr;
				VarULongArray	ulong_arr;
				VarLong64Array	long64_arr;
				VarULong64Array ulong64_arr;				
				VarFloatArray	float_arr;
				VarDoubleArray double_arr;
				VarStringArray string_arr;
				VarStateArray 	state_arr;
				VarEncodedArray encoded_arr;
      	  };
typedef union TangoAttributeData TangoAttributeData;
/**
 * \union TangoCommandData
 * An union of all Tango scalar and array data types used for command data reading and writing.
 */
union TangoCommandData {
				bool			  		bool_val;
				short					short_val;
				unsigned short			ushort_val;
				int				   		long_val;
				unsigned int 		  	ulong_val;
				float					float_val;
				double					double_val;
				char             		*string_val;
				TangoDevState	 		state_val;
				TangoDevLong64	 		long64_val;
				TangoDevULong64 		ulong64_val;
				VarBoolArray   			bool_arr;
				VarCharArray			char_arr;
				VarShortArray			short_arr;
				VarUShortArray			ushort_arr;
				VarLongArray			long_arr;
				VarULongArray			ulong_arr;
				VarLong64Array			long64_arr;
				VarULong64Array 		ulong64_arr;				
				VarFloatArray			float_arr;
				VarDoubleArray 			double_arr;
				VarStringArray 			string_arr;
				VarStateArray 			state_arr;
				TangoDevEncoded 		encoded_val;
				VarLongStringArray 		long_string_arr;
				VarDoubleStringArray 	double_string_arr;
      	  };
typedef union TangoCommandData TangoCommandData;
/**
 * \union TangoPropertyData
 * An union of all Tango scalar and array data types used for property reading and writing.
 */
union TangoPropertyData {
				bool			  		bool_val;
				unsigned char			char_val;
				short					short_val;
				unsigned short			ushort_val;
				int				   		long_val;
				unsigned int 		  	ulong_val;
				float					float_val;
				double					double_val;
				char             *string_val;
				TangoDevLong64	 long64_val;
				TangoDevULong64 ulong64_val;

				VarShortArray	short_arr;
				VarUShortArray	ushort_arr;
				VarLongArray	long_arr;
				VarULongArray	ulong_arr;
				VarLong64Array	long64_arr;
				VarULong64Array ulong64_arr;				
				VarFloatArray	float_arr;
				VarDoubleArray double_arr;
				VarStringArray string_arr;
      	  };
typedef union TangoPropertyData TangoPropertyData;

/* Command data structure */
/**
 * \struct CommandData
 * A structure containing the Tango data type and the command data union to 
 * transfer command data to and from a server.
 */
struct CommandData {
				TangoDataType		arg_type;	/**< Tango data type */	
				TangoCommandData  cmd_data;	/**< Union for command data */
			  };
typedef struct CommandData CommandData;

/* Attribute data structure */
/**
 * \struct AttributeData
 * A structure containing the scalar Tango data type and the attribute data union to 
 * transfer attribute data to and from a server.
 * The structure also contains the data dimension, the data quality and a time stamp when the
 * data was acquired.
 */
struct AttributeData {
				TangoDataType 	  data_type;	/**< Tango scalar data type */	
				TangoAttributeData attr_data;	/**< Union for attribute data */
				AttrDataFormat data_format;	/**< Data format (scalar, ...) */
				AttrQuality			quality;		/**< Data quality factor */
				long 				nb_read;		/**< Number of read items */
				char 				    *name;		 /**< Attribute name */
				int 					dim_x;		 /**< Data dimension X */
				int 					dim_y; 		 /**< Data dimension Y */
				struct timeval 	 time_stamp;  /**< Time stanp in seconds and milliseconds since epoch */
		  		};
typedef struct AttributeData AttributeData;
/**
 * \struct AttributeDataList
 * A structure containing a pointer to a sequence of attribute data structures 
 * and the number of elements in the sequence.
 */
struct AttributeDataList	{
				unsigned int 		length;
				AttributeData  *sequence;
				};
typedef struct AttributeDataList AttributeDataList;					


/* Error data structure */	
/**
 * \struct DevFailed
 * A structure that maps all fields of the Tango::DevFailed exception
 */
struct DevFailed {
				char 				*desc;		/**< Error description */
				char 				*reason;		/**< Error reason */
				char 				*origin;		/**< Error origin (class and method) */
				ErrSeverity 	severity;  /**< Error severity */
				};
typedef struct DevFailed DevFailed;
/**
 * \struct ErrorStack
 * A structure containing a pointer to a sequence of error structures and the number of elements in the sequence.
 */
struct ErrorStack	{
				unsigned int 	length;
				DevFailed  *sequence;
				};								
typedef struct ErrorStack ErrorStack;				
		
		
/* command query data structures */

/**
 * \struct CommandInfo
 * The command info structure contains descriptive command properties.
 */
struct CommandInfo {
				char 			*cmd_name;		/**< Command name string */
				int 			cmd_tag;			/**< Command as binary value (for TACO) */
				int 			in_type;			/**< in type as binary value */
				int 			out_type;		/**< out type as binary value */
				char 			*in_type_desc;	/**< description of in type (optional)*/
				char 			*out_type_desc;/**< description of out type (optional)*/
				DispLevel	disp_level;	  /**< Command display level */
				};
typedef struct CommandInfo CommandInfo;
/**
 * \struct CommandInfoList
 * A structure containing a pointer to a sequence of command info structures 
 * and the number of elements in the sequence.
 */
struct CommandInfoList	{
				unsigned int 		length;
				CommandInfo    *sequence;
				};
typedef struct CommandInfoList CommandInfoList;


/* attribute query data structures */

/**
 * \struct AttributeInfo
 * The attribute info structure contains descriptive attribute properties.
 */
struct AttributeInfo {
				char 				*name;				/**< Attribute name string */
				AttrWriteType 	writable;		  /**< Attribute type READ, WRITE, READ and WRITE */
				AttrDataFormat data_format;	  /**< scalar, 1D or 2D data */
				TangoDataType	data_type;		  /**<  The scalar Tango data type */
				int 				 max_dim_x;			/**< Maximum data size X */
				int 				 max_dim_y;			/**< Maximum data size Y */
				char 			   *description;		/**< Attribute description text */
				char 				*label;				/**< Attribute GUI label */
				char 				*unit;				/**< Attribute unit */
				char 				*standard_unit;	/**< Conversion factor to MKS unit */
				char 				*display_unit;		/**< Conversion factor to GUI display unit */
				char 				*format;				/**< Attribute diplay format (printf format) */
				char 				*min_value;			/**< Min value, checked when writing to an attribute */
				char 				*max_value;			/**< Max value, checked when writing to an attribute */
				char 				*min_alarm;			/**< Min alarm value, checked during state reading */
				char 				*max_alarm;			/**< Max alarm value, checked during state reading */
				char 				*writable_attr_name;	/**< Used only for READ_WTH_WRITE attributes */
				DispLevel 		disp_level;			/**< operator or expert display */
				};
typedef struct AttributeInfo AttributeInfo;				
/**
 * \struct AttributeInfoList
 * A structure containing a pointer to a sequence of attribute 
 * info structures and the number of elements in the sequence.
 */
struct AttributeInfoList {
				unsigned int 		length;
				AttributeInfo *sequence;
				};
typedef struct AttributeInfoList AttributeInfoList;		



/* Database data structure */

/**
 * \struct DbDatum
 * A container structure for the Tango database access.
 * All Tango query data and property releated data is passed with this structure.
 */
struct DbDatum {
				char					 *property_name; 	 /**< Name of the property */
				TangoDataType		data_type;			/**< Tango data type */	
				TangoPropertyData prop_data;			/**< Union for property data */
				
				bool 					 is_empty;			/**< set when no properties available */
				bool   				 wrong_data_type; /**< set when the property value cannot be
				                                                            converted to the given data type */
			  };
typedef struct DbDatum DbDatum;
/**
 * \struct DbData
 * A structure containing a pointer to a sequence of DbDatum structures
 * and the number of elements in the sequence.
 */
struct DbData {
				unsigned int 		length;
				DbDatum       	*sequence;
				};
typedef struct DbData DbData;
/*@}*/		
		
		
/* external function definitions */	

#ifdef __cplusplus
extern "C" {
#endif

extern const char* const TangoDataTypeName[];
extern const char* const TangoDevStateName[];

/**
 * \defgroup Proxy Tango Proxy Related Functions
 *
 * Functions to manipulate a device connection.
 */
/*@{*/
/** 
  * Create the access to a Tango device.
  *
  * \param[in] dev_name The name of the device to connect to.
  * \param[out] proxy The pointer to the device handle.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_create_device_proxy (char *dev_name, void **proxy, ErrorStack *error);
/** 
  * Delete the access to a Tango device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_delete_device_proxy (void **proxy, ErrorStack *error);
/** 
  * Set the timeout of a device connection. 
  * The timeout value is given in milliseconds.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] millis The timout value.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_set_timeout_millis  (void *proxy, int millis, ErrorStack *error);
/** 
  * Get the timeout of a device connection. 
  * The timeout value is given in milliseconds.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] millis The timout value.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_get_timeout_millis  (void *proxy, int *millis, ErrorStack *error);
/** 
  * Set the source for data reading.
  * Data can be read from the device or from the polling cache.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] source The data source to use.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_set_source          (void *proxy, DevSource source, ErrorStack *error);
/** 
  * Get the actual source for data reading.
  * Data can be read from the device or from the polling cache.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] source The actual data source.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_get_source          (void *proxy, DevSource *source, ErrorStack *error);
/** 
  * Lock a device.
  * A locked device is protected against the following calls when executed by another client:
  * Command_inout call, except for device state and status requested via command and for the set of
  * commands defined as allowed following the definition of allowed command in the Tango control
  * access schema,
  * write_attribute call,
  * write_read_attribute call and
  * set_attribute_config call
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_lock (void *proxy, ErrorStack *error);
/** 
  * Unlock a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_unlock (void *proxy, ErrorStack *error);
/** 
  * Checks the device lock.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] is_locked True when locked, otherwise false.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_is_locked (void *proxy, bool *is_locked, ErrorStack *error);
/** 
  * Checks whether the device lock is already taken by the caller.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] is_locked_by_me True when locked by the caller, otherwise false.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_is_locked_by_me (void *proxy, bool *is_locked_by_me, ErrorStack *error);
/** 
  * Get a locking status string.
  * The status contains detailed information on the process which has taken the lock.
  * Memory for the status string will be allocated and need to be freed.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] lock_status Lock status string.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_locking_status (void *proxy, char **lock_status, ErrorStack *error);
/*@}*/


/**
 * \defgroup Command Tango Command Related Functions
 *
 * Functions to query and execute Tango commands.
 */
/*@{*/
/** 
  * Query the descriptive command properties for a given command.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] cmd_name The name of the command.
  * \param[out] cmd_info Structure for descriptive command properties.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_command_query      (void *proxy, char *cmd_name, CommandInfo *cmd_info, ErrorStack *error);
/** 
  * Query the descriptive command properties for all commands of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] cmd_info_list A sequence of structures for descriptive command properties.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_command_list_query (void *proxy, CommandInfoList *cmd_info_list, ErrorStack *error);
/** 
  * Ececute Tango commands with input and output parameters.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] cmd_name The name of the command.
  * \param[in] argin The input parameters.
  * \param[out] argout The output parameters.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_command_inout      (void *proxy, char *cmd_name, CommandData *argin, CommandData *argout, ErrorStack *error);
/** 
  * Free the allocated command output data.
  *
  * \param[in] command_data The command data structure with allocated fields.
  */
void tango_free_CommandData   	(CommandData 		*command_data);
/** 
  * Free allocated command information.
  *
  * \param[in] command_info The command info structure with allocated fields.
  */
void tango_free_CommandInfo   	(CommandInfo 		*command_info);
/** 
  * Free the list of all allocated command information structures.
  *
  * \param[in] command_info_list The sequence of command info structures with allocated fields.
  */
void tango_free_CommandInfoList	(CommandInfoList 	*command_info_list);
/*@}*/

/**
 * \defgroup Attribute Tango Attribute Related Functions
 *
 * Functions to query, read and write Tango attributes.
 * Reading of attribute properties is also possible.
 */
/*@{*/
/** 
  * Get the names off all attributes of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] attr_names A string array with the attibute names.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_get_attribute_list   (void *proxy, VarStringArray *attr_names, ErrorStack *error);
/** 
  * Query the descriptive attribute properties for a list of attributes.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] attr_names The string array with the attribute names.
  * \param[out] attr_info_list A sequence of structures for descriptive attribute properties.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_get_attribute_config (void *proxy, VarStringArray *attr_names, AttributeInfoList *attr_info_list, ErrorStack *error);
/** 
  * Query the descriptive attribute properties for all attributes of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[out] attr_info_list A sequence of structures for descriptive attribute properties.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_attribute_list_query (void *proxy, AttributeInfoList *attr_info_list, ErrorStack *error);
/** 
  * Read data from one attribute of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] attr_name The attribute name.
  * \param[out] argout The read attribute data.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_read_attribute   (void *proxy, char *attr_name, AttributeData *argout, ErrorStack *error);
/** 
  * Write data to one attribute of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] argin The attribute data to be written.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_write_attribute  (void *proxy, AttributeData *argin, ErrorStack *error);
/** 
  * Read data from a list of attributes of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] attr_names The string array with the attribute names.
  * \param[out] argout A sequence of attribute data structures.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_read_attributes  (void *proxy, VarStringArray *attr_names, AttributeDataList *argout, ErrorStack *error);
/** 
  * Write data to a list of attributes of a device.
  *
  * \param[in] proxy The pointer to the device handle.
  * \param[in] argin A sequence of attribute data structures to be written.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_write_attributes (void *proxy, AttributeDataList *argin, ErrorStack *error);
/** 
  * Free the allocated attribute output data.
  *
  * \param[in] attribute_data The attribute data structure with allocated fields.
  */
void tango_free_AttributeData     (AttributeData 		*attribute_data);
/** 
  * Free the list of allocated attribute data structures.
  *
  * \param[in] attribute_data_list The sequence of attribute data structures with allocated fields.
  */
void tango_free_AttributeDataList (AttributeDataList 	*attribute_data_list);
/** 
  * Free the an allocated string array.
  *
  * \param[in] string_arr The allocated string array.
  */
void tango_free_VarStringArray    (VarStringArray     *string_arr);
/** 
  * Free the list of all allocated attribute information structures.
  *
  * \param[in] attribute_info_list The sequence of attribute info structures with allocated fields.
  */
void tango_free_AttributeInfoList (AttributeInfoList 	*attribute_info_list);
/*@}*/

/**
 * \defgroup Error Error Handling Related Functions
 *
 * Functions to print and free a Tango error stack.
 */
/*@{*/
/** 
  * Print an error stack to stdout.
  *
  * \param[in] error_stack The error stack of a Tango exception in case of failure.
  */
void tango_print_ErrorStack (ErrorStack *error_stack);
/** 
  * Free the data allocated for an error stack in case of a failure.
  *
  * \param[in] error_stack The error stack of a Tango exception in case of failure.
  */
void tango_free_ErrorStack  (ErrorStack *error_stack);
/*@}*/


/**
 * \defgroup Property Tango Property and Database Related Functions
 *
 * Functions to query, read and write Tango attributes.
 * Reading of attribute properties is also possible.
 */
/*@{*/
/** 
  * Create the access to the Tango database.
  * The function uses the environment variable ``TANGO_HOST'' to 
  * determine which instance of the TANGO database to connect to.
  *
  * \param[out] db_proxy The pointer to the database handle.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_create_database_proxy (void **db_proxy, ErrorStack *error);
/** 
  * Delete the access to the Tango database.
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_delete_database_proxy (void **db_proxy, ErrorStack *error);
/** 
  * Get a list of exported devices using a name filter.
  * The name filter can contain one or more wilcards (*).
  * Example:  sr/ *-pen/ *
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] name_filter The filter string
  * \param[out] dev_list DbDatum structure containing a string array with the list of exported devices.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_get_device_exported   (void *db_proxy, char *name_filter, 
                                 DbDatum *dev_list, ErrorStack *error);
/** 
  * Get a list of exported devices for a given Tango class.
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] class_name The name of the Tango class.
  * \param[out] dev_list DbDatum structure containing a string array with the list of exported devices.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */											
bool tango_get_device_exported_for_class   (void *db_proxy, char *class_name, 
                                    DbDatum *dev_list, ErrorStack *error);
/** 
  * Get a list of free property objects from the Tango database using a name filter.
  * The name filter can contain one or more wilcards (*).
  * Example:  my*prop/ *
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] name_filter The filter string
  * \param[out] obj_list DbDatum structure containing a string array with the list of free property objects.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */											
bool tango_get_object_list          (void *db_proxy, char *name_filter, 
                                    DbDatum *obj_list, ErrorStack *error);
/** 
  * Get a list of property names for a given free property object, using a name filter.
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] obj_name The name of the free property object.
  * \param[in] name_filter The property name filter string
  * \param[out] prop_list DbDatum structure containing a string array with the list of property names.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */													
bool tango_get_object_property_list (void *db_proxy, char *obj_name, char *name_filter, 
                                    DbDatum *prop_list, ErrorStack *error);
/** 
  * Get a list of properties for a given free property object.
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] obj_name The name of the free property object.
  * \param[out] prop_list A sequence of DbDatum structures containing the property names and the returned values.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */	
bool tango_get_property  	(void *db_proxy, char *obj_name, DbData *prop_list, ErrorStack *error);
/** 
  * Put a list of properties for a given free property object.
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] obj_name The name of the free property object.
  * \param[in] prop_list A sequence of DbDatum structures containing the properties to write.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */	
bool tango_put_property  	(void *db_proxy, char *obj_name, DbData *prop_list, ErrorStack *error);
/** 
  * Delete a list of properties for a given free property object.
  *
  * \param[in] db_proxy The pointer to the database handle.
  * \param[in] obj_name The name of the free property object.
  * \param[in] prop_list A sequence of DbDatum structures containing the property names to delete.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */	
bool tango_delete_property (void *db_proxy, char *obj_name, DbData *prop_list, ErrorStack *error);

/** 
  * Get a list of device properties.
  * The function uses the device handle and not the database handle.
  *
  * \param[in] dproxy The pointer to the device handle.
  * \param[out] prop_list A sequence of DbDatum structures containing the property  names and the returned values.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_get_device_property  	 (void *dev_proxy, DbData *prop_list, ErrorStack *error);
/** 
  * Put a list of device properties.
  * The function uses the device handle and not the database handle.
  *
  * \param[in] dproxy The pointer to the device handle.
  * \param[in] prop_list A sequence of DbDatum structures containing the properties to write.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_put_device_property  	 (void *dev_proxy, DbData *prop_list, ErrorStack *error);
/** 
  * Delete a list of device properties.
  * The function uses the device handle and not the database handle.
  *
  * \param[in] dproxy The pointer to the device handle.
  * \param[in] prop_list A sequence of DbDatum structures containing the property names to delete.
  * \param[out] error The error stack of a Tango exception in case of failure.
  * \return false when an failure was detected otherwise true.
  */
bool tango_delete_device_property (void *dev_proxy, DbData *prop_list, ErrorStack *error);
/** 
  * Free the allocated database data structure.
  *
  * \param[in] db_datum The returned database data structure with allocated fields.
  */
void tango_free_DbDatum   	(DbDatum *db_datum);
/** 
  * Free the list of all allocated database data structures.
  *
  * \param[in] db_data The sequence of returned database data structures with allocated fields.
  */
void tango_free_DbData   	(DbData  *db_data);
/*@}*/


#ifdef __cplusplus
}
#endif /*__cplusplus */
#endif /* C_TANGO_H */

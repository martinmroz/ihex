var searchIndex = JSON.parse('{\
"ihex":{"doc":"The IHEX Library","i":[[3,"ReaderOptions","ihex","",null,null],[12,"stop_after_first_error","","A flag indicating that iteration should stop on first…",0,null],[12,"stop_after_eof","","A flag indicating that iteration should stop on first EOF…",0,null],[3,"Reader","","",null,null],[4,"ReaderError","","",null,null],[13,"MissingStartCode","","The record provided does not begin with a \':\'.",1,null],[13,"RecordTooShort","","The record provided is shorter than the smallest valid.",1,null],[13,"RecordTooLong","","The record provided exceeds the maximum size (255b payload).",1,null],[13,"RecordNotEvenLength","","The record is not an even number of bytes.",1,null],[13,"ContainsInvalidCharacters","","The record is not all hexadecimal characters.",1,null],[13,"ChecksumMismatch","","The checksum did not match.",1,null],[13,"PayloadLengthMismatch","","The record is not the length it claims.",1,null],[13,"UnsupportedRecordType","","The record type is not supported.",1,null],[13,"InvalidLengthForType","","The payload length does not match the record type.",1,null],[4,"Record","","",null,null],[13,"Data","","Specifies a 16-bit offset address and up to 255 bytes of…",2,null],[12,"offset","ihex::Record","The offset of the data record in memory.",3,null],[12,"value","","Up to 255 bytes of data to be written to memory.",3,null],[13,"EndOfFile","ihex","Indicates the end of the object file. Must occur exactly…",2,null],[13,"ExtendedSegmentAddress","","Specifies bits 4-19 of the Segment Base Address (SBA) to…",2,null],[13,"StartSegmentAddress","","Specifies the 20-bit segment address via the CS and IP…",2,null],[12,"cs","ihex::Record","Value of the CS register.",4,null],[12,"ip","","Value of the IP register.",4,null],[13,"ExtendedLinearAddress","ihex","Specifies the upper 16 bits of a 32-bit linear address.…",2,null],[13,"StartLinearAddress","","Specifies the execution start address for the object file.…",2,null],[4,"WriterError","","",null,null],[13,"DataExceedsMaximumLength","","A record contains data too large to represent.",5,null],[13,"MissingEndOfFileRecord","","Object does not end in an EoF record.",5,null],[13,"MultipleEndOfFileRecords","","Object contains multiple EoF records.",5,null],[13,"SynthesisFailed","","Unable to synthesize record string.",5,null],[5,"checksum","","Computes the Intel HEX checksum of `data`. This is done by…",null,[[]]],[5,"create_object_file_representation","","Generates an Intel HEX object file representation of the…",null,[[],[["string",3],["writererror",4],["result",4]]]],[11,"from_record_string","","Constructs a new `Record` by parsing `string`.",2,[[],[["result",4],["readererror",4]]]],[11,"new_with_options","","Creates a new IHEX reader over `string` with the specified…",6,[[["readeroptions",3]]]],[11,"new","","Creates a new IHEX reader over `string` with default…",6,[[]]],[11,"record_type","","The record type specifier corresponding to the receiver.",2,[[]]],[11,"to_hex_string","","Returns the IHEX record representation of the receiver, or…",2,[[],[["string",3],["writererror",4],["result",4]]]],[0,"types","","",null,null],[17,"DATA","ihex::types","Type specifier for a Data record.",null,null],[17,"END_OF_FILE","","Type specifier for an End-Of-File record.",null,null],[17,"EXTENDED_SEGMENT_ADDRESS","","Type specifier for an Extended Segment Address record.",null,null],[17,"START_SEGMENT_ADDRESS","","Type specifier for a Start Segment Address record.",null,null],[17,"EXTENDED_LINEAR_ADDRESS","","Type specifier for an Extended Linear Address record.",null,null],[17,"START_LINEAR_ADDRESS","","Type specifier for a Start Linear Address record.",null,null],[11,"from","ihex","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"into_iter","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"to_string","","",5,[[],["string",3]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"next","","Iterates over the lines of the IHEX object, skipping any…",6,[[],["option",4]]],[11,"clone","","",1,[[],["readererror",4]]],[11,"clone","","",0,[[],["readeroptions",3]]],[11,"clone","","",2,[[],["record",4]]],[11,"clone","","",5,[[],["writererror",4]]],[11,"default","","",0,[[]]],[11,"eq","","",1,[[["readererror",4]]]],[11,"ne","","",1,[[["readererror",4]]]],[11,"eq","","",0,[[["readeroptions",3]]]],[11,"ne","","",0,[[["readeroptions",3]]]],[11,"eq","","",2,[[["record",4]]]],[11,"ne","","",2,[[["record",4]]]],[11,"eq","","",5,[[["writererror",4]]]],[11,"ne","","",5,[[["writererror",4]]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"hash","","",1,[[]]],[11,"hash","","",0,[[]]],[11,"hash","","",2,[[]]],[11,"hash","","",5,[[]]],[11,"from_str","","",2,[[],["result",4]]]],"p":[[3,"ReaderOptions"],[4,"ReaderError"],[4,"Record"],[13,"Data"],[13,"StartSegmentAddress"],[4,"WriterError"],[3,"Reader"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);
/// Request to close the connection, providing a reply code and text.
///
/// Either side may initiate a close. The peer must reply with
/// `connection.close-ok` before the TCP connection is torn down.

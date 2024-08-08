#!/usr/bin/python
from gui import *
import time

def send(timest, nick, msg):
    node_id = api.lookup_node_id("/window/view/chatty")

    arg_data = bytearray()
    serial.write_u64(arg_data, timest)
    arg_data += bytes(32)
    serial.encode_str(arg_data, nick)
    serial.encode_str(arg_data, msg)

    api.call_method(node_id, "insert_line", arg_data)

#for i in range(28):
for i in range(27):
    send(1722944640000 + i, "hhi12", "hello 123")
    time.sleep(0.1)
time.sleep(0.5)
send(1722944641015, "john", "hpqjyzzxxdhio")
time.sleep(1)
send(1722944642015, "clr", "hello1234ppjjh")

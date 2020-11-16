use rua_value_list::{VarList};
use rua_value_list::var_list::{Init, Get, Set};
use crate::values::{NetResult, make_extension_error};
use std::ops::Add;

fn get_msg_head_fill() -> VarList {
    VarList::init(vec![0 as u32, 0 as u16, 0 as u32, 0 as u16])
}

enum MsgIndex {
    MsgIndexLength,
    MsgIndexSeqFd,
    MsgIndexCookie,
    MsgIndexMsgType,
    MsgIndexMsgData,
}

pub struct NetMsg {
    var_list: VarList,
    seq_fd: u16,
    length: u32,
    cookie: u32,
    msg_type: u16,
    pack_name: String,
}

impl NetMsg {
    pub fn new() -> NetMsg {
        let mut var_list = VarList::new();
        var_list = var_list.combine(get_msg_head_fill());
        NetMsg {
            seq_fd: 0u16,
            length: var_list.get_count() as u32,
            cookie: 0u32,
            msg_type: 0u16,
            var_list,
            pack_name: String::new(),
        }
    }

    pub fn new_by_detail(msg_type: u16, msg_name: String, data: VarList) -> NetMsg {
        let mut var_list = VarList::new();
        var_list = var_list.combine(get_msg_head_fill())
            + msg_name.clone()
            + data.get_count() as u32;
        var_list = var_list.combine(data);

        let len = var_list.get_count();
        let mut net_msg = NetMsg {
            var_list,
            seq_fd: 0u16,
            length: len as u32,
            cookie: 0u32,
            msg_type: 0u16,
            pack_name: msg_name,
        };
        net_msg.end_msg(0);
        net_msg
    }

    pub fn new_by_data(data: VarList) -> NetResult<NetMsg> {
        if data.get_count() < get_msg_head_fill().get_count() {
            return Err(make_extension_error("data len too small", None));
        }

        let mut var_list = VarList::new();
        var_list = var_list.combine(data);
        var_list = var_list.add(0 as u32);
        let length: u32 = var_list.get(MsgIndex::MsgIndexLength.into());
        let seq_fd: u16 = var_list.get(MsgIndex::MsgIndexSeqFd.into());
        let cookie: u32 = var_list.get(MsgIndex::MsgIndexCookie.into());
        let msg_type: u16 = var_list.get(MsgIndex::MsgIndexMsgType.into());
        if data.get_count() != length as usize {
            println!("data.len() = {:?}, length = {:?}", data.len(), length);
            return Err(make_extension_error("data length not match", None));
        }
        let pack_name: String = var_list.get(MsgIndex::MsgIndexMsgData.into());
        Ok(NetMsg {
            seq_fd,
            length,
            cookie,
            msg_type,
            var_list,
            pack_name,
        })
    }

    pub fn min_len() -> usize {
        get_msg_head_fill().get_count()
    }

    pub fn end_msg(&mut self, seq_fd: u16) {
        self.seq_fd = seq_fd;
        self.length = self.var_list.get_count() as u32;
        self.var_list.set(MsgIndex::MsgIndexLength.into(), self.length);
        self.var_list.set(MsgIndex::MsgIndexSeqFd.into(), self.seq_fd);
        self.var_list.set(MsgIndex::MsgIndexCookie.into(), self.cookie);
        self.var_list.set(MsgIndex::MsgIndexMsgType.into(), self.msg_type);
    }

    pub fn get_var_list(&mut self) -> &mut VarList {
        &mut self.var_list
    }

    pub fn read_head(&mut self) -> NetResult<()> {
        self.length = self.var_list.get(MsgIndex::MsgIndexLength.into());
        self.seq_fd = self.var_list.get(MsgIndex::MsgIndexSeqFd.into());
        self.cookie = self.var_list.get(MsgIndex::MsgIndexCookie.into());
        self.msg_type = self.var_list.get(MsgIndex::MsgIndexMsgType.into());
        Ok(())
    }

    pub fn get_pack_len(&self) -> u32 {
        self.length
    }

    pub fn len(&self) -> usize {
        self.var_list.get_count()
    }

    pub fn sef_msg_type(&mut self, msg_type: u16) {
        self.msg_type = msg_type;
    }

    pub fn get_msg_type(&self) -> u16 {
        self.msg_type
    }

    pub fn set_seq_fd(&mut self, seq_fd: u16) {
        self.seq_fd = seq_fd;
        self.var_list.set(MsgIndex::MsgIndexSeqFd.into(), seq_fd);
    }

    pub fn get_seq_fd(&self) -> u16 {
        self.seq_fd
    }

    pub fn set_cookie(&mut self, cookie: u32) {
        self.cookie = cookie;
        self.var_list.set(MsgIndex::MsgIndexCookie.into(), cookie);
    }

    pub fn get_pack_name(&self) -> &String {
        &self.pack_name
    }
}
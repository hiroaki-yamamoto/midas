use ::kvs::kvs;

kvs!(pub, InitLock, String, "init_lock.{}");
kvs!(pub, InitFinLock, String, "init_fin_lock.{}");

MEMORY {
    RAM: start = $0000, size = $4000, type = rw;
    IO: start = $4000, size = $3f00, type = rw;
    ROM: start = $7F00, size = $80fa, type = ro, fill = yes;
    RESETVEC: start = $FFFA, size = 6, type = ro;
}

SEGMENTS {
    WOZMON: load = "ROM", type = ro;
    RESETVEC: load = "RESETVEC", type = ro;
}

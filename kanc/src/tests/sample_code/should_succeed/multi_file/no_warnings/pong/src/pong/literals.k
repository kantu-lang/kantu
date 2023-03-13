use super.*;

pub let pos1 = Pos.one;
pub let pos2 = pos.add(pos1, pos1);
pub let pos3 = pos.add(pos1, pos2);
pub let pos4 = pos.add(pos1, pos3);
pub let pos5 = pos.add(pos1, pos4);
pub let pos6 = pos.add(pos1, pos5);
pub let pos7 = pos.add(pos1, pos6);
pub let pos8 = pos.add(pos1, pos7);
pub let pos9 = pos.add(pos1, pos8);
pub let pos10 = pos.add(pos1, pos9);

pub let pos20 = pos.mul(pos2, pos10);
pub let pos30 = pos.mul(pos3, pos10);
pub let pos40 = pos.mul(pos4, pos10);
pub let pos50 = pos.mul(pos5, pos10);
pub let pos60 = pos.mul(pos6, pos10);
pub let pos70 = pos.mul(pos7, pos10);
pub let pos80 = pos.mul(pos8, pos10);
pub let pos90 = pos.mul(pos9, pos10);
pub let pos100 = pos.mul(pos10, pos10);

pub let pos64 = pos.mul(pos8, pos8);
pub let pos65 = pos.add(pos1, pos64);
pub let pos66 = pos.add(pos1, pos65);
pub let pos67 = pos.add(pos1, pos66);
pub let pos68 = pos.add(pos1, pos67);

pub let pos83 = pos.add(pos3, pos80);
pub let pos85 = pos.add(pos5, pos80);
pub let pos87 = pos.add(pos7, pos80);
pub let pos110 = pos.add(pos10, pos100);
pub let pos111 = pos.add(pos1, pos110);
pub let pos112 = pos.add(pos1, pos111);
pub let pos114 = pos.add(pos4, pos.add(pos10, pos100));
pub let pos115 = pos.add(pos5, pos.add(pos10, pos100));
pub let pos119 = pos.add(pos9, pos.add(pos10, pos100));

pub let pos1000 = pos.pow(pos10, pos3);
pub let pos5000 = pos.mul(pos5, pos1000);
pub let pos10k = pos.mul(pos10, pos1000);

pub let nat2 = Nat.pos(pos2);
pub let nat5000 = Nat.pos(pos5000);
pub let nat10k = Nat.pos(pos10k);

pub let u8_65 = U8.u8(Nat.pos(pos65), refl_true);
pub let u8_66 = U8.u8(Nat.pos(pos66), refl_true);
pub let u8_67 = U8.u8(Nat.pos(pos67), refl_true);
pub let u8_68 = U8.u8(Nat.pos(pos68), refl_true);
pub let u8_83 = U8.u8(Nat.pos(pos83), refl_true);
pub let u8_85 = U8.u8(Nat.pos(pos85), refl_true);
pub let u8_87 = U8.u8(Nat.pos(pos87), refl_true);
pub let u8_110 = U8.u8(Nat.pos(pos110), refl_true);
pub let u8_111 = U8.u8(Nat.pos(pos111), refl_true);
pub let u8_112 = U8.u8(Nat.pos(pos112), refl_true);
pub let u8_114 = U8.u8(Nat.pos(pos114), refl_true);
pub let u8_115 = U8.u8(Nat.pos(pos115), refl_true);
pub let u8_119 = U8.u8(Nat.pos(pos119), refl_true);

pub let ch_A = u8_65;
pub let ch_B = u8_66;
pub let ch_C = u8_67;
pub let ch_D = u8_68;
pub let ch_S = u8_83;
pub let ch_U = u8_85;
pub let ch_W = u8_87;

pub let ch_n = u8_110;
pub let ch_o = u8_111;
pub let ch_p = u8_112;
pub let ch_r = u8_114;
pub let ch_s = u8_115;
pub let ch_w = u8_119;

// "A"
pub let background_image_str = String.utf8(
    List.cons(
        U8,
        ch_A,
        List.nil(U8),
    ),
);
// "B"
pub let paddle_image_str = String.utf8(
    List.cons(
        U8,
        ch_B,
        List.nil(U8),
    ),
);
// "C"
pub let ball_image_str = String.utf8(
    List.cons(
        U8,
        ch_C,
        List.nil(U8),
    ),
);

// ["ArrowUp"]
pub let right_paddle_up_strs = List.cons(
    String,
    String.utf8(
        List.cons(
            U8,
            ch_A,
            List.cons(
                U8,
                ch_r,
                List.cons(
                    U8,
                    ch_r,
                    List.cons(
                        U8,
                        ch_o,
                        List.cons(
                            U8,
                            ch_w,
                            List.cons(
                                U8,
                                ch_U,
                                List.cons(
                                    U8,
                                    ch_p,
                                    List.nil(U8),
                                )
                            )
                        )
                    )
                )
            )
        )
    ),
    List.nil(String),
);

// ["ArrowDown"]
pub let right_paddle_down_strs = List.cons(
    String,
    String.utf8(
        List.cons(
            U8,
            ch_A,
            List.cons(
                U8,
                ch_r,
                List.cons(
                    U8,
                    ch_r,
                    List.cons(
                        U8,
                        ch_o,
                        List.cons(
                            U8,
                            ch_w,
                            List.cons(
                                U8,
                                ch_D,
                                List.cons(
                                    U8,
                                    ch_o,
                                    List.cons(
                                        U8,
                                        ch_w,
                                        List.cons(
                                            U8,
                                            ch_n,
                                            List.nil(U8),
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
    ),
    List.nil(String),
);


// ["w", "W"]
pub let left_paddle_up_strs = List.cons(
    String,
    String.utf8(
        List.cons(U8, ch_w, List.nil(U8))
    ),
    List.cons(
        String,
        String.utf8(
            List.cons(U8, ch_W, List.nil(U8))
        ),
        List.nil(String),
    )
);

// ["s", "S"]
pub let left_paddle_down_strs = List.cons(
    String,
    String.utf8(
        List.cons(U8, ch_s, List.nil(U8))
    ),
    List.cons(
        String,
        String.utf8(
            List.cons(U8, ch_S, List.nil(U8))
        ),
        List.nil(String),
    )
);

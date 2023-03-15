const Type1 = { type_species: "Type1" };

const Type = { type_species: "Type" };

const unreachable = function unreachable(unreachable_span) {
  throw new Error(
    "Reached supposedly unreachable path. This likely indicates that you passed one or more illegal arguments to one or more of the generated functions. Responsible span: " +
      unreachable_span
  );
};

const unimplemented = function unimplemented(unimplemented_span) {
  throw new Error(
    "This functionality is not implemented. Source span: " + unimplemented_span
  );
};

const Equal = function Equal(T, a, b) {
  return { type_species: "Equal", type_args: [T, a, b] };
};

const Equal_refl = function Equal_refl(T, a) {
  return ["refl", T, a];
};

const Bool = { type_species: "Bool", type_args: [] };

const Bool_true_ = ["true_"];

const Bool_false_ = ["false_"];

const not = function _(a) {
  return (function temp_1(temp_0) {
    if (temp_0[0] === "true_") {
      return Bool_false_;
    }
    if (temp_0[0] === "false_") {
      return Bool_true_;
    }
  })(a);
};

const and = function _(a, b) {
  return (function temp_3(temp_2) {
    if (temp_2[0] === "true_") {
      return b;
    }
    if (temp_2[0] === "false_") {
      return Bool_false_;
    }
  })(a);
};

const nand = function _(a, b) {
  return not(and(a, b));
};

const or = function _(a, b) {
  return (function temp_5(temp_4) {
    if (temp_4[0] === "true_") {
      return Bool_true_;
    }
    if (temp_4[0] === "false_") {
      return b;
    }
  })(a);
};

const nor = function _(a, b) {
  return not(or(a, b));
};

const xor = function _(a, b) {
  return (function temp_7(temp_6) {
    if (temp_6[0] === "true_") {
      return not(b);
    }
    if (temp_6[0] === "false_") {
      return b;
    }
  })(a);
};

const xnor = function _(a, b) {
  return not(xor(a, b));
};

const eq = xnor;

const neq = xor;

const Trueb = function _(a) {
  return Equal(Bool, Bool_true_, a);
};

const Falseb = function _(a) {
  return Equal(Bool, Bool_false_, a);
};

const Prod = function Prod(L, R) {
  return { type_species: "Prod", type_args: [L, R] };
};

const Prod_pair = function Prod_pair(L, l, R, r) {
  return ["pair", L, l, R, r];
};

const first = function _(L, R, p) {
  return (function temp_9(temp_8) {
    if (temp_8[0] === "pair") {
      const _2 = temp_8[1];
      const l = temp_8[2];
      const _3 = temp_8[3];
      const _4 = temp_8[4];
      return l;
    }
  })(p);
};

const second = function _(L, R, p) {
  return (function temp_b(temp_a) {
    if (temp_a[0] === "pair") {
      const _2 = temp_a[1];
      const _3 = temp_a[2];
      const _4 = temp_a[3];
      const r = temp_a[4];
      return r;
    }
  })(p);
};

const Sum = function Sum(L, R) {
  return { type_species: "Sum", type_args: [L, R] };
};

const Sum_inl = function Sum_inl(L, l, R) {
  return ["inl", L, l, R];
};

const Sum_inr = function Sum_inr(L, R, r) {
  return ["inr", L, R, r];
};

const Opt = function Opt(T) {
  return { type_species: "Opt", type_args: [T] };
};

const Opt_none = function Opt_none(T) {
  return ["none", T];
};

const Opt_some = function Opt_some(T, t) {
  return ["some", T, t];
};

const List = function List(T) {
  return { type_species: "List", type_args: [T] };
};

const List_nil = function List_nil(T) {
  return ["nil", T];
};

const List_cons = function List_cons(T, car, cdr) {
  return ["cons", T, car, cdr];
};

const eq2 = function eq2(T, a, b, eqf) {
  return (function temp_d(temp_c) {
    if (temp_c[0] === "nil") {
      const _ = temp_c[1];
      return (function temp_f(temp_e) {
        if (temp_e[0] === "nil") {
          const _2 = temp_e[1];
          return Bool_true_;
        }
        if (temp_e[0] === "cons") {
          const _2 = temp_e[1];
          const _3 = temp_e[2];
          const _4 = temp_e[3];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_c[0] === "cons") {
      const _ = temp_c[1];
      const a_car = temp_c[2];
      const a_cdr = temp_c[3];
      return (function temp_11(temp_10) {
        if (temp_10[0] === "nil") {
          const _2 = temp_10[1];
          return Bool_false_;
        }
        if (temp_10[0] === "cons") {
          const _2 = temp_10[1];
          const b_car = temp_10[2];
          const b_cdr = temp_10[3];
          return (function temp_13(temp_12) {
            if (temp_12[0] === "true_") {
              return eq2(T, a_cdr, b_cdr, eqf);
            }
            if (temp_12[0] === "false_") {
              return Bool_false_;
            }
          })(eqf(a_car, b_car));
        }
      })(b);
    }
  })(a);
};

const Bit = { type_species: "Bit", type_args: [] };

const Bit_zero = ["zero"];

const Bit_one = ["one"];

const Pos = { type_species: "Pos", type_args: [] };

const Pos_one = ["one"];

const Pos_extend = function Pos_extend(left, right) {
  return ["extend", left, right];
};

const Nat = { type_species: "Nat", type_args: [] };

const Nat_zero = ["zero"];

const Nat_pos = function Nat_pos(p) {
  return ["pos", p];
};

const Int = { type_species: "Int", type_args: [] };

const Int_neg = function Int_neg(n) {
  return ["neg", n];
};

const Int_nat = function Int_nat(n) {
  return ["nat", n];
};

const Sign = { type_species: "Sign", type_args: [] };

const Sign_pos = ["pos"];

const Sign_neg = ["neg"];

const Ord = { type_species: "Ord", type_args: [] };

const Ord_lt = ["lt"];

const Ord_eq = ["eq"];

const Ord_gt = ["gt"];

const is_zero = function _(a) {
  return (function temp_15(temp_14) {
    if (temp_14[0] === "zero") {
      return Bool_true_;
    }
    if (temp_14[0] === "one") {
      return Bool_false_;
    }
  })(a);
};

const is_one = function _(a) {
  return (function temp_17(temp_16) {
    if (temp_16[0] === "zero") {
      return Bool_false_;
    }
    if (temp_16[0] === "one") {
      return Bool_true_;
    }
  })(a);
};

const eq3 = function _(a, b) {
  return (function temp_19(temp_18) {
    if (temp_18[0] === "zero") {
      return is_zero(b);
    }
    if (temp_18[0] === "one") {
      return is_one(b);
    }
  })(a);
};

const neq2 = function _(a, b) {
  return not(eq3(a, b));
};

const eq4 = function eq4(a, b) {
  return (function temp_1b(temp_1a) {
    if (temp_1a[0] === "one") {
      return (function temp_1d(temp_1c) {
        if (temp_1c[0] === "one") {
          return Bool_true_;
        }
        if (temp_1c[0] === "extend") {
          const _ = temp_1c[1];
          const _2 = temp_1c[2];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_1a[0] === "extend") {
      const a_left = temp_1a[1];
      const a_right = temp_1a[2];
      return (function temp_1f(temp_1e) {
        if (temp_1e[0] === "one") {
          return Bool_false_;
        }
        if (temp_1e[0] === "extend") {
          const b_left = temp_1e[1];
          const b_right = temp_1e[2];
          return and(eq4(a_left, b_left), eq3(a_right, b_right));
        }
      })(b);
    }
  })(a);
};

const neq3 = function _(a, b) {
  return not(eq4(a, b));
};

const minimal_bitlist_plus = function minimal_bitlist_plus(
  a,
  right_accumulator
) {
  return (function temp_21(temp_20) {
    if (temp_20[0] === "one") {
      return List_cons(Bit, Bit_one, right_accumulator);
    }
    if (temp_20[0] === "extend") {
      const a_left = temp_20[1];
      const a_right = temp_20[2];
      return minimal_bitlist_plus(
        a_left,
        List_cons(Bit, a_right, right_accumulator)
      );
    }
  })(a);
};

const minimal_bitlist = function _(a) {
  return minimal_bitlist_plus(a, List_nil(Bit));
};

const succ = function succ(a) {
  return (function temp_23(temp_22) {
    if (temp_22[0] === "one") {
      return Pos_extend(Pos_one, Bit_zero);
    }
    if (temp_22[0] === "extend") {
      const a_left = temp_22[1];
      const a_right = temp_22[2];
      return (function temp_25(temp_24) {
        if (temp_24[0] === "zero") {
          return Pos_extend(a_left, Bit_one);
        }
        if (temp_24[0] === "one") {
          return Pos_extend(succ(a_left), Bit_zero);
        }
      })(a_right);
    }
  })(a);
};

const pred = function pred(a) {
  return (function temp_27(temp_26) {
    if (temp_26[0] === "one") {
      return Nat_zero;
    }
    if (temp_26[0] === "extend") {
      const a_left = temp_26[1];
      const a_right = temp_26[2];
      return Nat_pos(
        (function temp_29(temp_28) {
          if (temp_28[0] === "one") {
            return Pos_extend(a_left, Bit_zero);
          }
          if (temp_28[0] === "zero") {
            return (function temp_2b(temp_2a) {
              if (temp_2a[0] === "zero") {
                return Pos_one;
              }
              if (temp_2a[0] === "pos") {
                const a_left_pred = temp_2a[1];
                return Pos_extend(a_left_pred, Bit_one);
              }
            })(pred(a_left));
          }
        })(a_right)
      );
    }
  })(a);
};

const parity = function _(a) {
  return (function temp_2d(temp_2c) {
    if (temp_2c[0] === "one") {
      return Bit_one;
    }
    if (temp_2c[0] === "extend") {
      const _2 = temp_2c[1];
      const right = temp_2c[2];
      return right;
    }
  })(a);
};

const neg = Int_neg;

const add = function add(a, b) {
  return (function temp_2f(temp_2e) {
    if (temp_2e[0] === "one") {
      return succ(b);
    }
    if (temp_2e[0] === "extend") {
      const a_left = temp_2e[1];
      const a_right = temp_2e[2];
      return (function temp_31(temp_30) {
        if (temp_30[0] === "one") {
          return succ(a);
        }
        if (temp_30[0] === "extend") {
          const b_left = temp_30[1];
          const b_right = temp_30[2];
          return (function temp_33(temp_32) {
            if (temp_32[0] === "zero") {
              return Pos_extend(add(a_left, b_left), b_right);
            }
            if (temp_32[0] === "one") {
              return (function temp_35(temp_34) {
                if (temp_34[0] === "zero") {
                  return Pos_extend(add(a_left, b_left), Bit_one);
                }
                if (temp_34[0] === "one") {
                  return Pos_extend(succ(add(a_left, b_left)), Bit_zero);
                }
              })(b_right);
            }
          })(a_right);
        }
      })(b);
    }
  })(a);
};

const mul = function mul(a, b) {
  return (function temp_37(temp_36) {
    if (temp_36[0] === "one") {
      return b;
    }
    if (temp_36[0] === "extend") {
      const a_left = temp_36[1];
      const a_right = temp_36[2];
      return (function temp_39(temp_38) {
        if (temp_38[0] === "zero") {
          return Pos_extend(mul(a_left, b), Bit_zero);
        }
        if (temp_38[0] === "one") {
          return add(Pos_extend(mul(a_left, b), Bit_zero), b);
        }
      })(a_right);
    }
  })(a);
};

const square = function _(a) {
  return mul(a, a);
};

const pow = function pow(a, b) {
  return (function temp_3b(temp_3a) {
    if (temp_3a[0] === "one") {
      return a;
    }
    if (temp_3a[0] === "extend") {
      const b_left = temp_3a[1];
      const b_right = temp_3a[2];
      return (function temp_3d(temp_3c) {
        if (temp_3c[0] === "zero") {
          return square(pow(a, b_left));
        }
        if (temp_3c[0] === "one") {
          return mul(a, square(pow(a, b_left)));
        }
      })(b_right);
    }
  })(b);
};

const le = function le(a, b) {
  return (function temp_3f(temp_3e) {
    if (temp_3e[0] === "one") {
      return Bool_true_;
    }
    if (temp_3e[0] === "extend") {
      const a_left = temp_3e[1];
      const a_right = temp_3e[2];
      return (function temp_41(temp_40) {
        if (temp_40[0] === "one") {
          return Bool_false_;
        }
        if (temp_40[0] === "extend") {
          const b_left = temp_40[1];
          const b_right = temp_40[2];
          return (function temp_43(temp_42) {
            if (temp_42[0] === "zero") {
              return le(a_left, b_left);
            }
            if (temp_42[0] === "one") {
              return (function temp_45(temp_44) {
                if (temp_44[0] === "one") {
                  return le(a_left, b_left);
                }
                if (temp_44[0] === "zero") {
                  return and(le(a_left, b_left), neq3(a_left, b_left));
                }
              })(b_right);
            }
          })(a_right);
        }
      })(b);
    }
  })(a);
};

const lt = function _(a, b) {
  return and(le(a, b), neq3(a, b));
};

const ge = function _(a, b) {
  return le(b, a);
};

const gt = function _(a, b) {
  return lt(b, a);
};

const cmp = function _(a, b) {
  return (function temp_47(temp_46) {
    if (temp_46[0] === "true_") {
      return Ord_lt;
    }
    if (temp_46[0] === "false_") {
      return (function temp_49(temp_48) {
        if (temp_48[0] === "true_") {
          return Ord_eq;
        }
        if (temp_48[0] === "false_") {
          return Ord_gt;
        }
      })(eq4(a, b));
    }
  })(lt(a, b));
};

const min = function _(a, b) {
  return (function temp_4b(temp_4a) {
    if (temp_4a[0] === "true_") {
      return a;
    }
    if (temp_4a[0] === "false_") {
      return b;
    }
  })(lt(a, b));
};

const max = function _(a, b) {
  return (function temp_4d(temp_4c) {
    if (temp_4c[0] === "true_") {
      return a;
    }
    if (temp_4c[0] === "false_") {
      return b;
    }
  })(gt(a, b));
};

const one = Nat_pos(Pos_one);

const eq5 = function eq5(a, b) {
  return (function temp_4f(temp_4e) {
    if (temp_4e[0] === "zero") {
      return (function temp_51(temp_50) {
        if (temp_50[0] === "zero") {
          return Bool_true_;
        }
        if (temp_50[0] === "pos") {
          const _ = temp_50[1];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_4e[0] === "pos") {
      const ap = temp_4e[1];
      return (function temp_53(temp_52) {
        if (temp_52[0] === "zero") {
          return Bool_false_;
        }
        if (temp_52[0] === "pos") {
          const bp = temp_52[1];
          return eq4(ap, bp);
        }
      })(b);
    }
  })(a);
};

const neq4 = function _(a, b) {
  return not(eq5(a, b));
};

const succ2 = function succ2(a) {
  return (function temp_55(temp_54) {
    if (temp_54[0] === "zero") {
      return one;
    }
    if (temp_54[0] === "pos") {
      const ap = temp_54[1];
      return Nat_pos(succ(ap));
    }
  })(a);
};

const pred2 = function pred2(a) {
  return (function temp_57(temp_56) {
    if (temp_56[0] === "zero") {
      return Int_neg(Pos_one);
    }
    if (temp_56[0] === "pos") {
      const ap = temp_56[1];
      return Int_nat(pred(ap));
    }
  })(a);
};

const from_bit = function _(a) {
  return (function temp_59(temp_58) {
    if (temp_58[0] === "zero") {
      return Nat_zero;
    }
    if (temp_58[0] === "one") {
      return one;
    }
  })(a);
};

const extend_right = function _(a, right) {
  return (function temp_5b(temp_5a) {
    if (temp_5a[0] === "zero") {
      return from_bit(right);
    }
    if (temp_5a[0] === "pos") {
      const ap = temp_5a[1];
      return Nat_pos(Pos_extend(ap, right));
    }
  })(a);
};

const extend_right_with_bits = function extend_right_with_bits(a, right) {
  return (function temp_5d(temp_5c) {
    if (temp_5c[0] === "nil") {
      const _ = temp_5c[1];
      return a;
    }
    if (temp_5c[0] === "cons") {
      const _ = temp_5c[1];
      const car = temp_5c[2];
      const cdr = temp_5c[3];
      return extend_right_with_bits(extend_right(a, car), cdr);
    }
  })(right);
};

const from_bitlist = function _(bits) {
  return extend_right_with_bits(Nat_zero, bits);
};

const parity2 = function _(a) {
  return (function temp_5f(temp_5e) {
    if (temp_5e[0] === "zero") {
      return Bit_zero;
    }
    if (temp_5e[0] === "pos") {
      const ap = temp_5e[1];
      return parity(ap);
    }
  })(a);
};

const neg2 = function _(a) {
  return (function temp_61(temp_60) {
    if (temp_60[0] === "zero") {
      return Int_nat(Nat_zero);
    }
    if (temp_60[0] === "pos") {
      const ap = temp_60[1];
      return Int_neg(ap);
    }
  })(a);
};

const add2 = function add2(a, b) {
  return (function temp_63(temp_62) {
    if (temp_62[0] === "zero") {
      return b;
    }
    if (temp_62[0] === "pos") {
      const ap = temp_62[1];
      return (function temp_65(temp_64) {
        if (temp_64[0] === "zero") {
          return a;
        }
        if (temp_64[0] === "pos") {
          const bp = temp_64[1];
          return Nat_pos(add(ap, bp));
        }
      })(b);
    }
  })(a);
};

const mul2 = function mul2(a, b) {
  return (function temp_67(temp_66) {
    if (temp_66[0] === "zero") {
      return Nat_zero;
    }
    if (temp_66[0] === "pos") {
      const ap = temp_66[1];
      return (function temp_69(temp_68) {
        if (temp_68[0] === "zero") {
          return Nat_zero;
        }
        if (temp_68[0] === "pos") {
          const bp = temp_68[1];
          return Nat_pos(mul(ap, bp));
        }
      })(b);
    }
  })(a);
};

const square2 = function _(a) {
  return mul2(a, a);
};

const pow2 = function _(a, b) {
  return (function temp_6b(temp_6a) {
    if (temp_6a[0] === "zero") {
      return one;
    }
    if (temp_6a[0] === "pos") {
      const bp = temp_6a[1];
      return (function temp_6d(temp_6c) {
        if (temp_6c[0] === "zero") {
          return Nat_zero;
        }
        if (temp_6c[0] === "pos") {
          const ap = temp_6c[1];
          return Nat_pos(pow(ap, bp));
        }
      })(a);
    }
  })(b);
};

const le2 = function le2(a, b) {
  return (function temp_6f(temp_6e) {
    if (temp_6e[0] === "zero") {
      return Bool_true_;
    }
    if (temp_6e[0] === "pos") {
      const ap = temp_6e[1];
      return (function temp_71(temp_70) {
        if (temp_70[0] === "zero") {
          return Bool_false_;
        }
        if (temp_70[0] === "pos") {
          const bp = temp_70[1];
          return le(ap, bp);
        }
      })(b);
    }
  })(a);
};

const lt2 = function _(a, b) {
  return and(le2(a, b), neq4(a, b));
};

const ge2 = function _(a, b) {
  return le2(b, a);
};

const gt2 = function _(a, b) {
  return lt2(b, a);
};

const cmp2 = function _(a, b) {
  return (function temp_73(temp_72) {
    if (temp_72[0] === "true_") {
      return Ord_lt;
    }
    if (temp_72[0] === "false_") {
      return (function temp_75(temp_74) {
        if (temp_74[0] === "true_") {
          return Ord_eq;
        }
        if (temp_74[0] === "false_") {
          return Ord_gt;
        }
      })(eq5(a, b));
    }
  })(lt2(a, b));
};

const min2 = function _(a, b) {
  return (function temp_77(temp_76) {
    if (temp_76[0] === "true_") {
      return a;
    }
    if (temp_76[0] === "false_") {
      return b;
    }
  })(lt2(a, b));
};

const max2 = function _(a, b) {
  return (function temp_79(temp_78) {
    if (temp_78[0] === "true_") {
      return a;
    }
    if (temp_78[0] === "false_") {
      return b;
    }
  })(gt2(a, b));
};

const one2 = Int_nat(Nat_pos(Pos_one));

const eq6 = function eq6(a, b) {
  return (function temp_7b(temp_7a) {
    if (temp_7a[0] === "neg") {
      const neg_a = temp_7a[1];
      return (function temp_7d(temp_7c) {
        if (temp_7c[0] === "neg") {
          const neg_b = temp_7c[1];
          return eq4(neg_a, neg_b);
        }
        if (temp_7c[0] === "nat") {
          const _ = temp_7c[1];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_7a[0] === "nat") {
      const an = temp_7a[1];
      return (function temp_7f(temp_7e) {
        if (temp_7e[0] === "neg") {
          const _ = temp_7e[1];
          return Bool_false_;
        }
        if (temp_7e[0] === "nat") {
          const bn = temp_7e[1];
          return eq5(an, bn);
        }
      })(b);
    }
  })(a);
};

const neq5 = function _(a, b) {
  return not(eq6(a, b));
};

const succ3 = function _(a) {
  return (function temp_81(temp_80) {
    if (temp_80[0] === "neg") {
      const neg_a = temp_80[1];
      return neg2(pred(neg_a));
    }
    if (temp_80[0] === "nat") {
      const an = temp_80[1];
      return Int_nat(succ2(an));
    }
  })(a);
};

const pred3 = function _(a) {
  return (function temp_83(temp_82) {
    if (temp_82[0] === "neg") {
      const neg_a = temp_82[1];
      return Int_neg(succ(neg_a));
    }
    if (temp_82[0] === "nat") {
      const an = temp_82[1];
      return pred2(an);
    }
  })(a);
};

const sign = function _(a) {
  return (function temp_85(temp_84) {
    if (temp_84[0] === "neg") {
      const _2 = temp_84[1];
      return Opt_some(Sign, Sign_neg);
    }
    if (temp_84[0] === "nat") {
      const an = temp_84[1];
      return (function temp_87(temp_86) {
        if (temp_86[0] === "zero") {
          return Opt_none(Sign);
        }
        if (temp_86[0] === "pos") {
          const _2 = temp_86[1];
          return Opt_some(Sign, Sign_pos);
        }
      })(an);
    }
  })(a);
};

const parity3 = function _(a) {
  return (function temp_89(temp_88) {
    if (temp_88[0] === "neg") {
      const neg_a = temp_88[1];
      return parity(neg_a);
    }
    if (temp_88[0] === "nat") {
      const an = temp_88[1];
      return parity2(an);
    }
  })(a);
};

const neg3 = function _(a) {
  return (function temp_8b(temp_8a) {
    if (temp_8a[0] === "neg") {
      const neg_a = temp_8a[1];
      return Int_nat(Nat_pos(neg_a));
    }
    if (temp_8a[0] === "nat") {
      const an = temp_8a[1];
      return neg2(an);
    }
  })(a);
};

const double_ = function _(a) {
  return (function temp_8d(temp_8c) {
    if (temp_8c[0] === "neg") {
      const neg_a = temp_8c[1];
      return Int_neg(Pos_extend(neg_a, Bit_zero));
    }
    if (temp_8c[0] === "nat") {
      const an = temp_8c[1];
      return (function temp_8f(temp_8e) {
        if (temp_8e[0] === "zero") {
          return a;
        }
        if (temp_8e[0] === "pos") {
          const ap = temp_8e[1];
          return Int_nat(Nat_pos(Pos_extend(ap, Bit_zero)));
        }
      })(an);
    }
  })(a);
};

const sub_pos = function sub_pos(a, b) {
  return (function temp_91(temp_90) {
    if (temp_90[0] === "one") {
      return Int_nat(pred(a));
    }
    if (temp_90[0] === "extend") {
      const b_left = temp_90[1];
      const b_right = temp_90[2];
      return (function temp_93(temp_92) {
        if (temp_92[0] === "one") {
          return neg2(pred(b));
        }
        if (temp_92[0] === "extend") {
          const a_left = temp_92[1];
          const a_right = temp_92[2];
          return (function temp_95(temp_94) {
            if (temp_94[0] === "zero") {
              return (function temp_97(temp_96) {
                if (temp_96[0] === "zero") {
                  return double_(sub_pos(a_left, b_left));
                }
                if (temp_96[0] === "one") {
                  return succ3(double_(sub_pos(a_left, b_left)));
                }
              })(a_right);
            }
            if (temp_94[0] === "one") {
              return (function temp_99(temp_98) {
                if (temp_98[0] === "one") {
                  return double_(sub_pos(a_left, b_left));
                }
                if (temp_98[0] === "zero") {
                  return pred3(double_(sub_pos(a_left, b_left)));
                }
              })(a_right);
            }
          })(b_right);
        }
      })(a);
    }
  })(b);
};

const add3 = function _(a, b) {
  return (function temp_9b(temp_9a) {
    if (temp_9a[0] === "neg") {
      const neg_a = temp_9a[1];
      return (function temp_9d(temp_9c) {
        if (temp_9c[0] === "neg") {
          const neg_b = temp_9c[1];
          return Int_neg(add(neg_a, neg_b));
        }
        if (temp_9c[0] === "nat") {
          const bn = temp_9c[1];
          return (function temp_9f(temp_9e) {
            if (temp_9e[0] === "zero") {
              return a;
            }
            if (temp_9e[0] === "pos") {
              const bp = temp_9e[1];
              return sub_pos(bp, neg_a);
            }
          })(bn);
        }
      })(b);
    }
    if (temp_9a[0] === "nat") {
      const an = temp_9a[1];
      return (function temp_a1(temp_a0) {
        if (temp_a0[0] === "zero") {
          return b;
        }
        if (temp_a0[0] === "pos") {
          const ap = temp_a0[1];
          return (function temp_a3(temp_a2) {
            if (temp_a2[0] === "neg") {
              const neg_b = temp_a2[1];
              return sub_pos(ap, neg_b);
            }
            if (temp_a2[0] === "nat") {
              const bn = temp_a2[1];
              return Int_nat(add2(an, bn));
            }
          })(b);
        }
      })(an);
    }
  })(a);
};

const sub = function _(a, b) {
  return add3(a, neg3(b));
};

const mul3 = function _(a, b) {
  return (function temp_a5(temp_a4) {
    if (temp_a4[0] === "neg") {
      const neg_a = temp_a4[1];
      return (function temp_a7(temp_a6) {
        if (temp_a6[0] === "neg") {
          const neg_b = temp_a6[1];
          return Int_nat(Nat_pos(mul(neg_a, neg_b)));
        }
        if (temp_a6[0] === "nat") {
          const bn = temp_a6[1];
          return neg2(mul2(Nat_pos(neg_a), bn));
        }
      })(b);
    }
    if (temp_a4[0] === "nat") {
      const an = temp_a4[1];
      return (function temp_a9(temp_a8) {
        if (temp_a8[0] === "neg") {
          const neg_b = temp_a8[1];
          return neg2(mul2(an, Nat_pos(neg_b)));
        }
        if (temp_a8[0] === "nat") {
          const bn = temp_a8[1];
          return Int_nat(mul2(an, bn));
        }
      })(b);
    }
  })(a);
};

const trunc_div_bitlist = function trunc_div_bitlist(
  dividend_left,
  dividend_right,
  divisor
) {
  return (function temp_ab(temp_aa) {
    if (temp_aa[0] === "nil") {
      const _ = temp_aa[1];
      return List_nil(Bit);
    }
    if (temp_aa[0] === "cons") {
      const _ = temp_aa[1];
      const car = temp_aa[2];
      const cdr = temp_aa[3];
      return (function temp_ad(temp_ac) {
        if (temp_ac[0] === "neg") {
          const _2 = temp_ac[1];
          return List_cons(
            Bit,
            Bit_zero,
            trunc_div_bitlist(extend_right(dividend_left, car), cdr, divisor)
          );
        }
        if (temp_ac[0] === "nat") {
          const remainder = temp_ac[1];
          return List_cons(
            Bit,
            Bit_one,
            trunc_div_bitlist(remainder, cdr, divisor)
          );
        }
      })(
        sub(
          Int_nat(extend_right(dividend_left, car)),
          Int_nat(Nat_pos(divisor))
        )
      );
    }
  })(dividend_right);
};

const trunc_div_pos = function _(a, b) {
  return from_bitlist(trunc_div_bitlist(Nat_zero, minimal_bitlist(a), b));
};

const trunc_div_nat = function _(a, b) {
  return (function temp_af(temp_ae) {
    if (temp_ae[0] === "zero") {
      return Nat_zero;
    }
    if (temp_ae[0] === "pos") {
      const ap = temp_ae[1];
      return trunc_div_pos(ap, b);
    }
  })(a);
};

const trunc_div = function _(a, b) {
  return (function temp_b1(temp_b0) {
    if (temp_b0[0] === "neg") {
      const neg_a = temp_b0[1];
      return neg2(trunc_div_pos(neg_a, b));
    }
    if (temp_b0[0] === "nat") {
      const an = temp_b0[1];
      return Int_nat(trunc_div_nat(an, b));
    }
  })(a);
};

const trunc_div_signed_divisor = function _(a, b_mag, b_sign) {
  return (function temp_b3(temp_b2) {
    if (temp_b2[0] === "pos") {
      return trunc_div(a, b_mag);
    }
    if (temp_b2[0] === "neg") {
      return neg3(trunc_div(a, b_mag));
    }
  })(b_sign);
};

const square3 = function _(a) {
  return mul3(a, a);
};

const pow3 = function _(a, b) {
  return (function temp_b5(temp_b4) {
    if (temp_b4[0] === "zero") {
      return one2;
    }
    if (temp_b4[0] === "pos") {
      const bp = temp_b4[1];
      return (function temp_b7(temp_b6) {
        if (temp_b6[0] === "neg") {
          const neg_a = temp_b6[1];
          return (function temp_b9(temp_b8) {
            if (temp_b8[0] === "zero") {
              return Int_nat(Nat_pos(pow(neg_a, bp)));
            }
            if (temp_b8[0] === "one") {
              return Int_neg(pow(neg_a, bp));
            }
          })(parity(bp));
        }
        if (temp_b6[0] === "nat") {
          const an = temp_b6[1];
          return Int_nat(pow2(an, b));
        }
      })(a);
    }
  })(b);
};

const le3 = function _(a, b) {
  return (function temp_bb(temp_ba) {
    if (temp_ba[0] === "neg") {
      const neg_a = temp_ba[1];
      return (function temp_bd(temp_bc) {
        if (temp_bc[0] === "nat") {
          const _2 = temp_bc[1];
          return Bool_true_;
        }
        if (temp_bc[0] === "neg") {
          const neg_b = temp_bc[1];
          return ge(neg_a, neg_b);
        }
      })(b);
    }
    if (temp_ba[0] === "nat") {
      const an = temp_ba[1];
      return (function temp_bf(temp_be) {
        if (temp_be[0] === "neg") {
          const _2 = temp_be[1];
          return Bool_false_;
        }
        if (temp_be[0] === "nat") {
          const bn = temp_be[1];
          return le2(an, bn);
        }
      })(b);
    }
  })(a);
};

const lt3 = function _(a, b) {
  return and(le3(a, b), neq5(a, b));
};

const ge3 = function _(a, b) {
  return le3(b, a);
};

const gt3 = function _(a, b) {
  return lt3(b, a);
};

const cmp3 = function _(a, b) {
  return (function temp_c1(temp_c0) {
    if (temp_c0[0] === "true_") {
      return Ord_lt;
    }
    if (temp_c0[0] === "false_") {
      return (function temp_c3(temp_c2) {
        if (temp_c2[0] === "true_") {
          return Ord_eq;
        }
        if (temp_c2[0] === "false_") {
          return Ord_gt;
        }
      })(eq6(a, b));
    }
  })(lt3(a, b));
};

const min3 = function _(a, b) {
  return (function temp_c5(temp_c4) {
    if (temp_c4[0] === "true_") {
      return a;
    }
    if (temp_c4[0] === "false_") {
      return b;
    }
  })(lt3(a, b));
};

const max3 = function _(a, b) {
  return (function temp_c7(temp_c6) {
    if (temp_c6[0] === "true_") {
      return a;
    }
    if (temp_c6[0] === "false_") {
      return b;
    }
  })(gt3(a, b));
};

const pos_2 = add(Pos_one, Pos_one);

const pos_3 = add(pos_2, Pos_one);

const pos_5 = add(pos_2, pos_3);

const pos_10 = add(pos_5, pos_5);

const pos_50 = mul(pos_5, pos_10);

const pos_100 = mul(pos_10, pos_10);

const pos_200 = mul(pos_100, pos_2);

const pos_255 = add(pos_200, add(pos_50, pos_5));

const nat_255 = Nat_pos(pos_255);

const U8 = { type_species: "U8", type_args: [] };

const U8_u8 = function U8_u8(n, upper) {
  return ["u8", n, upper];
};

const eq7 = function _(a, b) {
  return (function temp_c9(temp_c8) {
    if (temp_c8[0] === "u8") {
      const an = temp_c8[1];
      const _2 = temp_c8[2];
      return (function temp_cb(temp_ca) {
        if (temp_ca[0] === "u8") {
          const bn = temp_ca[1];
          const _3 = temp_ca[2];
          return eq5(an, bn);
        }
      })(b);
    }
  })(a);
};

const Time = { type_species: "Time", type_args: [] };

const Time_time = function Time_time(millis_since_epoch) {
  return ["time", millis_since_epoch];
};

const String = { type_species: "String", type_args: [] };

const String_utf8 = function String_utf8(bytes) {
  return ["utf8", bytes];
};

const eq8 = function _(s1, s2) {
  return (function temp_cd(temp_cc) {
    if (temp_cc[0] === "utf8") {
      const s1_bytes = temp_cc[1];
      return (function temp_cf(temp_ce) {
        if (temp_ce[0] === "utf8") {
          const s2_bytes = temp_ce[1];
          return eq2(U8, s1_bytes, s2_bytes, eq7);
        }
      })(s2);
    }
  })(s1);
};

const Entity = { type_species: "Entity", type_args: [] };

const Entity_unscaled = function Entity_unscaled(x, y, image) {
  return ["unscaled", x, y, image];
};

const Entity_scaled = function Entity_scaled(x, y, w, h, image) {
  return ["scaled", x, y, w, h, image];
};

const Window = { type_species: "Window", type_args: [] };

const Window_window = function Window_window(w, h) {
  return ["window", w, h];
};

const Event = { type_species: "Event", type_args: [] };

const Event_keydown = function Event_keydown(key) {
  return ["keydown", key];
};

const Event_keyup = function Event_keyup(key) {
  return ["keyup", key];
};

const Event_window_resize = function Event_window_resize(window) {
  return ["window_resize", window];
};

const App = { type_species: "App", type_args: [] };

const App_app = function App_app(State, render, tick, init, handle) {
  return ["app", State, render, tick, init, handle];
};

const LocalX = { type_species: "LocalX", type_args: [] };

const LocalX_locx = function LocalX_locx(x) {
  return ["locx", x];
};

const LocalY = { type_species: "LocalY", type_args: [] };

const LocalY_locy = function LocalY_locy(y) {
  return ["locy", y];
};

const locx_raw = function _(lx) {
  return (function temp_d1(temp_d0) {
    if (temp_d0[0] === "locx") {
      const x = temp_d0[1];
      return x;
    }
  })(lx);
};

const locy_raw = function _(ly) {
  return (function temp_d3(temp_d2) {
    if (temp_d2[0] === "locy") {
      const y = temp_d2[1];
      return y;
    }
  })(ly);
};

const IntLocalX = { type_species: "IntLocalX", type_args: [] };

const IntLocalX_ilocx = function IntLocalX_ilocx(x) {
  return ["ilocx", x];
};

const IntLocalY = { type_species: "IntLocalY", type_args: [] };

const IntLocalY_ilocy = function IntLocalY_ilocy(y) {
  return ["ilocy", y];
};

const ilocx_raw = function _(lx) {
  return (function temp_d5(temp_d4) {
    if (temp_d4[0] === "ilocx") {
      const x = temp_d4[1];
      return x;
    }
  })(lx);
};

const ilocy_raw = function _(ly) {
  return (function temp_d7(temp_d6) {
    if (temp_d6[0] === "ilocy") {
      const y = temp_d6[1];
      return y;
    }
  })(ly);
};

const refl_true = Equal_refl(Bool, Bool_true_);

const identity = function _(T, t) {
  return t;
};

const ascribe = identity;

const str_list_contains = function str_list_contains(strs, str) {
  return (function temp_d9(temp_d8) {
    if (temp_d8[0] === "nil") {
      const _ = temp_d8[1];
      return Bool_false_;
    }
    if (temp_d8[0] === "cons") {
      const _ = temp_d8[1];
      const car = temp_d8[2];
      const cdr = temp_d8[3];
      return (function temp_db(temp_da) {
        if (temp_da[0] === "true_") {
          return Bool_true_;
        }
        if (temp_da[0] === "false_") {
          return str_list_contains(cdr, str);
        }
      })(eq8(car, str));
    }
  })(strs);
};

const sign_eq = function _(a, b) {
  return (function temp_dd(temp_dc) {
    if (temp_dc[0] === "pos") {
      return (function temp_df(temp_de) {
        if (temp_de[0] === "pos") {
          return Bool_true_;
        }
        if (temp_de[0] === "neg") {
          return Bool_false_;
        }
      })(b);
    }
    if (temp_dc[0] === "neg") {
      return (function temp_e1(temp_e0) {
        if (temp_e0[0] === "pos") {
          return Bool_false_;
        }
        if (temp_e0[0] === "neg") {
          return Bool_true_;
        }
      })(b);
    }
  })(a);
};

const opt_sign_eq_some = function _(opt_sign, sign2) {
  return (function temp_e3(temp_e2) {
    if (temp_e2[0] === "none") {
      const _2 = temp_e2[1];
      return Bool_false_;
    }
    if (temp_e2[0] === "some") {
      const _2 = temp_e2[1];
      const sign22 = temp_e2[2];
      return sign_eq(sign2, sign22);
    }
  })(opt_sign);
};

const sign_int = function _(sign2, i) {
  return (function temp_e5(temp_e4) {
    if (temp_e4[0] === "pos") {
      return i;
    }
    if (temp_e4[0] === "neg") {
      return neg3(i);
    }
  })(sign2);
};

const relu = function _(n) {
  return (function temp_e7(temp_e6) {
    if (temp_e6[0] === "neg") {
      const _2 = temp_e6[1];
      return Nat_zero;
    }
    if (temp_e6[0] === "nat") {
      const nn = temp_e6[1];
      return nn;
    }
  })(n);
};

const int_to_pos = function _(n) {
  return (function temp_e9(temp_e8) {
    if (temp_e8[0] === "neg") {
      const _2 = temp_e8[1];
      return Pos_one;
    }
    if (temp_e8[0] === "nat") {
      const nn = temp_e8[1];
      return (function temp_eb(temp_ea) {
        if (temp_ea[0] === "zero") {
          return Pos_one;
        }
        if (temp_ea[0] === "pos") {
          const np = temp_ea[1];
          return np;
        }
      })(nn);
    }
  })(n);
};

const nat_to_pos = function _(n) {
  return (function temp_ed(temp_ec) {
    if (temp_ec[0] === "zero") {
      return Pos_one;
    }
    if (temp_ec[0] === "pos") {
      const p = temp_ec[1];
      return p;
    }
  })(n);
};

const nat_sub = function _(min4, sub2) {
  return relu(sub(Int_nat(min4), Int_nat(sub2)));
};

const sign_nat = function _(s, n) {
  return (function temp_ef(temp_ee) {
    if (temp_ee[0] === "pos") {
      return Int_nat(n);
    }
    if (temp_ee[0] === "neg") {
      return neg2(n);
    }
  })(s);
};

const nat_mod = function _(n, divisor) {
  return relu(
    sub(
      Int_nat(n),
      mul3(trunc_div(Int_nat(n), divisor), Int_nat(Nat_pos(divisor)))
    )
  );
};

const int_abs = function _(i) {
  return (function temp_f1(temp_f0) {
    if (temp_f0[0] === "neg") {
      const neg_i = temp_f0[1];
      return Nat_pos(neg_i);
    }
    if (temp_f0[0] === "nat") {
      const in_ = temp_f0[1];
      return in_;
    }
  })(i);
};

const nat_dist = function _(a, b) {
  return int_abs(sub(Int_nat(a), Int_nat(b)));
};

const clamp = function _({ clampee: clampee, min: min4, max: max4 }) {
  return (function temp_f3(temp_f2) {
    if (temp_f2[0] === "true_") {
      return min4;
    }
    if (temp_f2[0] === "false_") {
      return (function temp_f5(temp_f4) {
        if (temp_f4[0] === "true_") {
          return max4;
        }
        if (temp_f4[0] === "false_") {
          return clampee;
        }
      })(ge2(clampee, max4));
    }
  })(le2(clampee, min4));
};

const sign_neg_one_exp = function _(n) {
  return (function temp_f7(temp_f6) {
    if (temp_f6[0] === "zero") {
      return Sign_pos;
    }
    if (temp_f6[0] === "one") {
      return Sign_neg;
    }
  })(parity2(n));
};

const window_width = function _(window) {
  return (function temp_f9(temp_f8) {
    if (temp_f8[0] === "window") {
      const w = temp_f8[1];
      const _2 = temp_f8[2];
      return w;
    }
  })(window);
};

const window_height = function _(window) {
  return (function temp_fb(temp_fa) {
    if (temp_fa[0] === "window") {
      const _2 = temp_fa[1];
      const h = temp_fa[2];
      return h;
    }
  })(window);
};

const time_millis = function _(t) {
  return (function temp_fd(temp_fc) {
    if (temp_fc[0] === "time") {
      const millis = temp_fc[1];
      return millis;
    }
  })(t);
};

const pos1 = Pos_one;

const pos2 = add(pos1, pos1);

const pos3 = add(pos1, pos2);

const pos4 = add(pos1, pos3);

const pos5 = add(pos1, pos4);

const pos6 = add(pos1, pos5);

const pos7 = add(pos1, pos6);

const pos8 = add(pos1, pos7);

const pos9 = add(pos1, pos8);

const pos10 = add(pos1, pos9);

const pos20 = mul(pos2, pos10);

const pos30 = mul(pos3, pos10);

const pos40 = mul(pos4, pos10);

const pos50 = mul(pos5, pos10);

const pos60 = mul(pos6, pos10);

const pos70 = mul(pos7, pos10);

const pos80 = mul(pos8, pos10);

const pos90 = mul(pos9, pos10);

const pos100 = mul(pos10, pos10);

const pos32 = add(pos2, pos30);

const pos64 = add(pos4, pos60);

const pos65 = add(pos5, pos60);

const pos66 = add(pos6, pos60);

const pos67 = add(pos7, pos60);

const pos68 = add(pos8, pos60);

const pos83 = add(pos3, pos80);

const pos85 = add(pos5, pos80);

const pos87 = add(pos7, pos80);

const pos97 = add(pos7, pos90);

const pos99 = add(pos9, pos90);

const pos101 = add(pos1, pos100);

const pos110 = add(pos10, pos100);

const pos111 = add(pos1, pos110);

const pos112 = add(pos1, pos111);

const pos114 = add(pos4, add(pos10, pos100));

const pos115 = add(pos5, add(pos10, pos100));

const pos119 = add(pos9, add(pos10, pos100));

const pos1000 = pow(pos10, pos3);

const pos5000 = mul(pos5, pos1000);

const pos10k = mul(pos10, pos1000);

const pos20k = mul(pos20, pos1000);

const nat1 = Nat_pos(Pos_one);

const nat2 = Nat_pos(pos2);

const nat5000 = Nat_pos(pos5000);

const nat10k = Nat_pos(pos10k);

const int1 = Int_nat(nat1);

const int0 = Int_nat(Nat_zero);

const int10k = Int_nat(nat10k);

const u8_32 = U8_u8(Nat_pos(pos32), refl_true);

const u8_65 = U8_u8(Nat_pos(pos65), refl_true);

const u8_66 = U8_u8(Nat_pos(pos66), refl_true);

const u8_67 = U8_u8(Nat_pos(pos67), refl_true);

const u8_68 = U8_u8(Nat_pos(pos68), refl_true);

const u8_83 = U8_u8(Nat_pos(pos83), refl_true);

const u8_85 = U8_u8(Nat_pos(pos85), refl_true);

const u8_87 = U8_u8(Nat_pos(pos87), refl_true);

const u8_97 = U8_u8(Nat_pos(pos97), refl_true);

const u8_99 = U8_u8(Nat_pos(pos99), refl_true);

const u8_101 = U8_u8(Nat_pos(pos101), refl_true);

const u8_110 = U8_u8(Nat_pos(pos110), refl_true);

const u8_111 = U8_u8(Nat_pos(pos111), refl_true);

const u8_112 = U8_u8(Nat_pos(pos112), refl_true);

const u8_114 = U8_u8(Nat_pos(pos114), refl_true);

const u8_115 = U8_u8(Nat_pos(pos115), refl_true);

const u8_119 = U8_u8(Nat_pos(pos119), refl_true);

const ch_A = u8_65;

const ch_B = u8_66;

const ch_C = u8_67;

const ch_D = u8_68;

const ch_S = u8_83;

const ch_U = u8_85;

const ch_W = u8_87;

const ch_a = u8_97;

const ch_c = u8_99;

const ch_e = u8_101;

const ch_n = u8_110;

const ch_o = u8_111;

const ch_p = u8_112;

const ch_r = u8_114;

const ch_s = u8_115;

const ch_w = u8_119;

const ch_space = u8_32;

const background_image_str = String_utf8(List_cons(U8, ch_A, List_nil(U8)));

const paddle_image_str = String_utf8(List_cons(U8, ch_B, List_nil(U8)));

const ball_image_str = String_utf8(List_cons(U8, ch_C, List_nil(U8)));

const right_paddle_up_strs = List_cons(
  String,
  String_utf8(
    List_cons(
      U8,
      ch_A,
      List_cons(
        U8,
        ch_r,
        List_cons(
          U8,
          ch_r,
          List_cons(
            U8,
            ch_o,
            List_cons(
              U8,
              ch_w,
              List_cons(U8, ch_U, List_cons(U8, ch_p, List_nil(U8)))
            )
          )
        )
      )
    )
  ),
  List_nil(String)
);

const right_paddle_down_strs = List_cons(
  String,
  String_utf8(
    List_cons(
      U8,
      ch_A,
      List_cons(
        U8,
        ch_r,
        List_cons(
          U8,
          ch_r,
          List_cons(
            U8,
            ch_o,
            List_cons(
              U8,
              ch_w,
              List_cons(
                U8,
                ch_D,
                List_cons(
                  U8,
                  ch_o,
                  List_cons(U8, ch_w, List_cons(U8, ch_n, List_nil(U8)))
                )
              )
            )
          )
        )
      )
    )
  ),
  List_nil(String)
);

const left_paddle_up_strs = List_cons(
  String,
  String_utf8(List_cons(U8, ch_w, List_nil(U8))),
  List_cons(
    String,
    String_utf8(List_cons(U8, ch_W, List_nil(U8))),
    List_nil(String)
  )
);

const left_paddle_down_strs = List_cons(
  String,
  String_utf8(List_cons(U8, ch_s, List_nil(U8))),
  List_cons(
    String,
    String_utf8(List_cons(U8, ch_S, List_nil(U8))),
    List_nil(String)
  )
);

const launch_ball_strs = List_cons(
  String,
  String_utf8(List_cons(U8, ch_space, List_nil(U8))),
  List_nil(String)
);

const paddle_height = LocalY_locy(trunc_div_nat(nat10k, pos7));

const ball_height = LocalY_locy(trunc_div_nat(nat10k, pos64));

const paddle_x_margin = LocalX_locx(trunc_div_nat(nat10k, pos64));

const paddle_width = LocalX_locx(trunc_div_nat(nat10k, pos64));

const left_paddle_x = paddle_x_margin;

const right_paddle_x = LocalX_locx(
  nat_sub(nat_sub(nat10k, locx_raw(paddle_x_margin)), locx_raw(paddle_width))
);

const ball_initial_vx = IntLocalX_ilocx(trunc_div(Int_nat(nat10k), pos4));

const ball_initial_vy = IntLocalY_ilocy(trunc_div(Int_nat(nat10k), pos3));

const paddle_max_y = LocalY_locy(nat_sub(nat10k, locy_raw(paddle_height)));

const clamp_paddle_y = function _(yi) {
  return (function temp_ff(temp_fe) {
    if (temp_fe[0] === "true_") {
      return LocalY_locy(Nat_zero);
    }
    if (temp_fe[0] === "false_") {
      return (function temp_101(temp_100) {
        if (temp_100[0] === "true_") {
          return paddle_max_y;
        }
        if (temp_100[0] === "false_") {
          return LocalY_locy(relu(yi));
        }
      })(gt3(yi, Int_nat(locy_raw(paddle_max_y))));
    }
  })(lt3(yi, int0));
};

const to_real_x = function _(x, window) {
  return (function temp_103(temp_102) {
    if (temp_102[0] === "window") {
      const window_w = temp_102[1];
      const _2 = temp_102[2];
      return trunc_div_nat(mul2(locx_raw(x), window_w), pos10k);
    }
  })(window);
};

const to_real_y = function _(y, window) {
  return (function temp_105(temp_104) {
    if (temp_104[0] === "window") {
      const _2 = temp_104[1];
      const window_h = temp_104[2];
      return trunc_div_nat(mul2(locy_raw(y), window_h), pos10k);
    }
  })(window);
};

const State = { type_species: "State", type_args: [] };

const State_state = function State_state({
  time: time,
  window: window,
  left_paddle_y: left_paddle_y,
  right_paddle_y: right_paddle_y,
  ball_center_x: ball_center_x,
  ball_center_y: ball_center_y,
  ball_vx: ball_vx,
  ball_vy: ball_vy,
  left_paddle_vy_sign: left_paddle_vy_sign,
  right_paddle_vy_sign: right_paddle_vy_sign,
}) {
  return [
    "state",
    {
      time: time,
      window: window,
      left_paddle_y: left_paddle_y,
      right_paddle_y: right_paddle_y,
      ball_center_x: ball_center_x,
      ball_center_y: ball_center_y,
      ball_vx: ball_vx,
      ball_vy: ball_vy,
      left_paddle_vy_sign: left_paddle_vy_sign,
      right_paddle_vy_sign: right_paddle_vy_sign,
    },
  ];
};

const init = function _(window, time) {
  return (function temp_107(temp_106) {
    if (temp_106[0] === "window") {
      const window_w = temp_106[1];
      const window_h = temp_106[2];
      return State_state({
        time: time,
        window: window,
        left_paddle_y: LocalY_locy(
          nat_sub(nat5000, trunc_div_nat(locy_raw(paddle_height), pos2))
        ),
        right_paddle_y: LocalY_locy(
          nat_sub(nat5000, trunc_div_nat(locy_raw(paddle_height), pos2))
        ),
        ball_center_x: LocalX_locx(nat5000),
        ball_center_y: LocalY_locy(nat5000),
        ball_vx: ball_initial_vx,
        ball_vy: ball_initial_vy,
        left_paddle_vy_sign: Opt_none(Sign),
        right_paddle_vy_sign: Opt_none(Sign),
      });
    }
  })(window);
};

const render_background = function _(state) {
  return (function temp_109(temp_108) {
    if (temp_108[0] === "state") {
      const window = temp_108[1].window;
      return (function temp_10b(temp_10a) {
        if (temp_10a[0] === "window") {
          const window_w = temp_10a[1];
          const window_h = temp_10a[2];
          return Entity_scaled(
            Nat_zero,
            Nat_zero,
            window_w,
            window_h,
            background_image_str
          );
        }
      })(window);
    }
  })(state);
};

const render_left_paddle = function _(state) {
  return (function temp_10d(temp_10c) {
    if (temp_10c[0] === "state") {
      const window = temp_10c[1].window;
      const left_paddle_y = temp_10c[1].left_paddle_y;
      return Entity_scaled(
        to_real_x(left_paddle_x, window),
        to_real_y(left_paddle_y, window),
        to_real_x(paddle_width, window),
        to_real_y(paddle_height, window),
        paddle_image_str
      );
    }
  })(state);
};

const render_right_paddle = function _(state) {
  return (function temp_10f(temp_10e) {
    if (temp_10e[0] === "state") {
      const window = temp_10e[1].window;
      const right_paddle_y = temp_10e[1].right_paddle_y;
      return Entity_scaled(
        to_real_x(right_paddle_x, window),
        to_real_y(right_paddle_y, window),
        to_real_x(paddle_width, window),
        to_real_y(paddle_height, window),
        paddle_image_str
      );
    }
  })(state);
};

const get_ball_width = function _(state) {
  return (function temp_111(temp_110) {
    if (temp_110[0] === "state") {
      const window = temp_110[1].window;
      return LocalX_locx(
        trunc_div_nat(
          mul2(locy_raw(ball_height), window_height(window)),
          nat_to_pos(window_width(window))
        )
      );
    }
  })(state);
};

const render_ball = function _(state) {
  return (function temp_113(temp_112) {
    if (temp_112[0] === "state") {
      const window = temp_112[1].window;
      const ball_center_x = temp_112[1].ball_center_x;
      const ball_center_y = temp_112[1].ball_center_y;
      return Entity_scaled(
        nat_sub(
          to_real_x(ball_center_x, window),
          trunc_div_nat(to_real_x(get_ball_width(state), window), pos2)
        ),
        nat_sub(
          to_real_y(ball_center_y, window),
          trunc_div_nat(to_real_y(ball_height, window), pos2)
        ),
        to_real_x(get_ball_width(state), window),
        to_real_y(ball_height, window),
        ball_image_str
      );
    }
  })(state);
};

const render = function _(state) {
  return List_cons(
    Entity,
    render_background(state),
    List_cons(
      Entity,
      render_left_paddle(state),
      List_cons(
        Entity,
        render_right_paddle(state),
        List_cons(Entity, render_ball(state), List_nil(Entity))
      )
    )
  );
};

const ball_bounce_y_top = function _(raw) {
  return sub(
    int10k,
    Int_nat(int_abs(sub(Int_nat(nat_mod(int_abs(raw), pos20k)), int10k)))
  );
};

const ball_bounce_y = function _(old_y, delta_y) {
  return LocalY_locy(
    relu(
      add3(
        ball_bounce_y_top(
          sub(
            add3(Int_nat(locy_raw(old_y)), ilocy_raw(delta_y)),
            Int_nat(trunc_div_nat(locy_raw(ball_height), pos2))
          )
        ),
        Int_nat(trunc_div_nat(locy_raw(ball_height), pos2))
      )
    )
  );
};

const ball_bounce_vy_sign_top = function _(raw) {
  return sign_neg_one_exp(
    (function temp_115(temp_114) {
      if (temp_114[0] === "zero") {
        return Nat_zero;
      }
      if (temp_114[0] === "pos") {
        const p = temp_114[1];
        return add2(
          trunc_div_nat(nat_sub(Nat_pos(p), nat1), pos10k),
          (function temp_117(temp_116) {
            if (temp_116[0] === "neg") {
              const _2 = temp_116[1];
              return one;
            }
            if (temp_116[0] === "nat") {
              const _2 = temp_116[1];
              return Nat_zero;
            }
          })(raw)
        );
      }
    })(int_abs(raw))
  );
};

const ball_bounce_vy_sign = function _(old_y, delta_y) {
  return ball_bounce_vy_sign_top(
    sub(
      add3(Int_nat(locy_raw(old_y)), ilocy_raw(delta_y)),
      Int_nat(trunc_div_nat(locy_raw(ball_height), pos2))
    )
  );
};

const Rect = { type_species: "Rect", type_args: [] };

const Rect_new_ = function Rect_new_(x, y, w, h) {
  return ["new_", x, y, w, h];
};

const Circ = { type_species: "Circ", type_args: [] };

const Circ_new_ = function Circ_new_(x, y, r) {
  return ["new_", x, y, r];
};

const HitBox = { type_species: "HitBox", type_args: [] };

const HitBox_rect = function HitBox_rect(r) {
  return ["rect", r];
};

const HitBox_circ = function HitBox_circ(c) {
  return ["circ", c];
};

const do_rects_intersect = function _(a, b) {
  return (function temp_119(temp_118) {
    if (temp_118[0] === "new_") {
      const ax = temp_118[1];
      const ay = temp_118[2];
      const aw = temp_118[3];
      const ah = temp_118[4];
      return (function temp_11b(temp_11a) {
        if (temp_11a[0] === "new_") {
          const bx = temp_11a[1];
          const by = temp_11a[2];
          const bw = temp_11a[3];
          const bh = temp_11a[4];
          return not(
            or(
              gt2(locx_raw(ax), add2(locx_raw(bx), locx_raw(bw))),
              or(
                gt2(locx_raw(bx), add2(locx_raw(ax), locx_raw(aw))),
                or(
                  gt2(locy_raw(ay), add2(locy_raw(by), locy_raw(bh))),
                  gt2(locy_raw(by), add2(locy_raw(ay), locy_raw(ah)))
                )
              )
            )
          );
        }
      })(b);
    }
  })(a);
};

const do_rect_circ_intersect = function _(rect, circ) {
  return (function temp_11d(temp_11c) {
    if (temp_11c[0] === "new_") {
      const rx = temp_11c[1];
      const ry = temp_11c[2];
      const rw = temp_11c[3];
      const rh = temp_11c[4];
      return (function temp_11f(temp_11e) {
        if (temp_11e[0] === "new_") {
          const cx = temp_11e[1];
          const cy = temp_11e[2];
          const cr = temp_11e[3];
          return le2(
            add2(
              square2(
                nat_dist(
                  locx_raw(cx),
                  clamp({
                    clampee: locx_raw(cx),
                    min: locx_raw(rx),
                    max: add2(locx_raw(rx), locx_raw(rw)),
                  })
                )
              ),
              square2(
                nat_dist(
                  locy_raw(cy),
                  clamp({
                    clampee: locy_raw(cy),
                    min: locy_raw(ry),
                    max: add2(locy_raw(ry), locy_raw(rh)),
                  })
                )
              )
            ),
            square2(locx_raw(cr))
          );
        }
      })(circ);
    }
  })(rect);
};

const do_circs_intersect = function _(a, b) {
  return (function temp_121(temp_120) {
    if (temp_120[0] === "new_") {
      const ax = temp_120[1];
      const ay = temp_120[2];
      const ar = temp_120[3];
      return (function temp_123(temp_122) {
        if (temp_122[0] === "new_") {
          const bx = temp_122[1];
          const by = temp_122[2];
          const br = temp_122[3];
          return le2(
            add2(
              square2(nat_dist(locx_raw(ax), locx_raw(bx))),
              square2(nat_dist(locy_raw(ay), locy_raw(by)))
            ),
            square2(add2(locx_raw(ar), locx_raw(br)))
          );
        }
      })(b);
    }
  })(a);
};

const does_intersect = function _(a, b) {
  return (function temp_125(temp_124) {
    if (temp_124[0] === "rect") {
      const ar = temp_124[1];
      return (function temp_127(temp_126) {
        if (temp_126[0] === "rect") {
          const br = temp_126[1];
          return do_rects_intersect(ar, br);
        }
        if (temp_126[0] === "circ") {
          const bc = temp_126[1];
          return do_rect_circ_intersect(ar, bc);
        }
      })(b);
    }
    if (temp_124[0] === "circ") {
      const ac = temp_124[1];
      return (function temp_129(temp_128) {
        if (temp_128[0] === "rect") {
          const br = temp_128[1];
          return do_rect_circ_intersect(br, ac);
        }
        if (temp_128[0] === "circ") {
          const bc = temp_128[1];
          return do_circs_intersect(ac, bc);
        }
      })(b);
    }
  })(a);
};

const ball_hitbox = function _(state) {
  return (function temp_12b(temp_12a) {
    if (temp_12a[0] === "state") {
      const ball_center_x = temp_12a[1].ball_center_x;
      const ball_center_y = temp_12a[1].ball_center_y;
      return HitBox_rect(
        Rect_new_(
          LocalX_locx(
            nat_sub(
              locx_raw(ball_center_x),
              trunc_div_nat(locx_raw(get_ball_width(state)), pos2)
            )
          ),
          LocalY_locy(
            nat_sub(
              locy_raw(ball_center_y),
              trunc_div_nat(locy_raw(ball_height), pos2)
            )
          ),
          get_ball_width(state),
          ball_height
        )
      );
    }
  })(state);
};

const left_paddle_hitbox = function _(state) {
  return (function temp_12d(temp_12c) {
    if (temp_12c[0] === "state") {
      const left_paddle_y = temp_12c[1].left_paddle_y;
      return HitBox_rect(
        Rect_new_(left_paddle_x, left_paddle_y, paddle_width, paddle_height)
      );
    }
  })(state);
};

const right_paddle_hitbox = function _(state) {
  return (function temp_12f(temp_12e) {
    if (temp_12e[0] === "state") {
      const right_paddle_y = temp_12e[1].right_paddle_y;
      return HitBox_rect(
        Rect_new_(right_paddle_x, right_paddle_y, paddle_width, paddle_height)
      );
    }
  })(state);
};

const is_ball_touching_left_paddle = function _(state) {
  return does_intersect(ball_hitbox(state), left_paddle_hitbox(state));
};

const is_ball_touching_right_paddle = function _(state) {
  return does_intersect(ball_hitbox(state), right_paddle_hitbox(state));
};

const tick_assuming_ball_not_reset = function _(state, new_time) {
  return (function temp_131(temp_130) {
    if (temp_130[0] === "state") {
      const window = temp_130[1].window;
      const left_paddle_vy_sign = temp_130[1].left_paddle_vy_sign;
      const right_paddle_vy_sign = temp_130[1].right_paddle_vy_sign;
      const old_time = temp_130[1].time;
      const old_left_paddle_y = temp_130[1].left_paddle_y;
      const old_right_paddle_y = temp_130[1].right_paddle_y;
      const old_ball_center_x = temp_130[1].ball_center_x;
      const old_ball_center_y = temp_130[1].ball_center_y;
      const old_ball_vx = temp_130[1].ball_vx;
      const old_ball_vy = temp_130[1].ball_vy;
      return (function _2({ elapsed_millis: elapsed_millis }) {
        return State_state({
          window: window,
          left_paddle_vy_sign: left_paddle_vy_sign,
          right_paddle_vy_sign: right_paddle_vy_sign,
          time: new_time,
          left_paddle_y: old_left_paddle_y,
          right_paddle_y: old_right_paddle_y,
          ball_center_x: LocalX_locx(
            relu(
              add3(
                Int_nat(locx_raw(old_ball_center_x)),
                trunc_div(
                  mul3(ilocx_raw(old_ball_vx), Int_nat(elapsed_millis)),
                  pos1000
                )
              )
            )
          ),
          ball_center_y: ball_bounce_y(
            old_ball_center_y,
            IntLocalY_ilocy(
              trunc_div(
                mul3(ilocy_raw(old_ball_vy), Int_nat(elapsed_millis)),
                pos1000
              )
            )
          ),
          ball_vx: (function temp_133(temp_132) {
            if (temp_132[0] === "true_") {
              return IntLocalX_ilocx(Int_nat(int_abs(ilocx_raw(old_ball_vx))));
            }
            if (temp_132[0] === "false_") {
              return (function temp_135(temp_134) {
                if (temp_134[0] === "true_") {
                  return IntLocalX_ilocx(
                    neg3(Int_nat(int_abs(ilocx_raw(old_ball_vx))))
                  );
                }
                if (temp_134[0] === "false_") {
                  return old_ball_vx;
                }
              })(is_ball_touching_right_paddle(state));
            }
          })(is_ball_touching_left_paddle(state)),
          ball_vy: IntLocalY_ilocy(
            sign_int(
              ball_bounce_vy_sign(
                old_ball_center_y,
                IntLocalY_ilocy(
                  trunc_div(
                    mul3(ilocy_raw(old_ball_vy), Int_nat(elapsed_millis)),
                    pos1000
                  )
                )
              ),
              ilocy_raw(old_ball_vy)
            )
          ),
        });
      })({
        elapsed_millis: nat_sub(time_millis(new_time), time_millis(old_time)),
      });
    }
  })(state);
};

const get_elapsed_millis = function _(state, new_time) {
  return (function temp_137(temp_136) {
    if (temp_136[0] === "state") {
      const old_time = temp_136[1].time;
      return nat_sub(time_millis(new_time), time_millis(old_time));
    }
  })(state);
};

const will_ball_go_out_of_x_bounds = function _(state, new_time) {
  return (function temp_139(temp_138) {
    if (temp_138[0] === "state") {
      const ball_center_x = temp_138[1].ball_center_x;
      const ball_vx = temp_138[1].ball_vx;
      return (function _2(ball_x) {
        return or(lt3(ilocx_raw(ball_x), int0), gt3(ilocx_raw(ball_x), int10k));
      })(
        IntLocalX_ilocx(
          add3(
            Int_nat(locx_raw(ball_center_x)),
            trunc_div(
              mul3(
                ilocx_raw(ball_vx),
                Int_nat(get_elapsed_millis(state, new_time))
              ),
              pos1000
            )
          )
        )
      );
    }
  })(state);
};

const reset_ball = function _(state) {
  return (function temp_13b(temp_13a) {
    if (temp_13a[0] === "state") {
      const window = temp_13a[1].window;
      const time = temp_13a[1].time;
      const left_paddle_y = temp_13a[1].left_paddle_y;
      const right_paddle_y = temp_13a[1].right_paddle_y;
      const ball_center_x = temp_13a[1].ball_center_x;
      const ball_center_y = temp_13a[1].ball_center_y;
      const ball_vx = temp_13a[1].ball_vx;
      const ball_vy = temp_13a[1].ball_vy;
      const left_paddle_vy_sign = temp_13a[1].left_paddle_vy_sign;
      const right_paddle_vy_sign = temp_13a[1].right_paddle_vy_sign;
      return State_state({
        ball_vx: IntLocalX_ilocx(int0),
        ball_vy: IntLocalY_ilocy(int0),
        ball_center_x: LocalX_locx(nat5000),
        ball_center_y: LocalY_locy(nat5000),
        window: window,
        time: time,
        left_paddle_y: left_paddle_y,
        right_paddle_y: right_paddle_y,
        left_paddle_vy_sign: left_paddle_vy_sign,
        right_paddle_vy_sign: right_paddle_vy_sign,
      });
    }
  })(state);
};

const tick = function _(state, new_time) {
  return (function temp_13d(temp_13c) {
    if (temp_13c[0] === "true_") {
      return reset_ball(state);
    }
    if (temp_13c[0] === "false_") {
      return tick_assuming_ball_not_reset(state, new_time);
    }
  })(will_ball_go_out_of_x_bounds(state, new_time));
};

const handle_window_resize = function _(state, new_window) {
  return (function temp_13f(temp_13e) {
    if (temp_13e[0] === "state") {
      const old_window = temp_13e[1].window;
      const time = temp_13e[1].time;
      const left_paddle_y = temp_13e[1].left_paddle_y;
      const right_paddle_y = temp_13e[1].right_paddle_y;
      const ball_center_x = temp_13e[1].ball_center_x;
      const ball_center_y = temp_13e[1].ball_center_y;
      const ball_vx = temp_13e[1].ball_vx;
      const ball_vy = temp_13e[1].ball_vy;
      const left_paddle_vy_sign = temp_13e[1].left_paddle_vy_sign;
      const right_paddle_vy_sign = temp_13e[1].right_paddle_vy_sign;
      return State_state({
        window: new_window,
        time: time,
        left_paddle_y: left_paddle_y,
        right_paddle_y: right_paddle_y,
        ball_center_x: ball_center_x,
        ball_center_y: ball_center_y,
        ball_vx: ball_vx,
        ball_vy: ball_vy,
        left_paddle_vy_sign: left_paddle_vy_sign,
        right_paddle_vy_sign: right_paddle_vy_sign,
      });
    }
  })(state);
};

const is_ball_stationary_at_center = function _(state) {
  return (function temp_141(temp_140) {
    if (temp_140[0] === "state") {
      const ball_center_x = temp_140[1].ball_center_x;
      const ball_center_y = temp_140[1].ball_center_y;
      const ball_vx = temp_140[1].ball_vx;
      const ball_vy = temp_140[1].ball_vy;
      return and(
        eq6(int0, ilocx_raw(ball_vx)),
        and(
          eq6(int0, ilocy_raw(ball_vy)),
          and(
            eq5(nat5000, locx_raw(ball_center_x)),
            eq5(nat5000, locy_raw(ball_center_y))
          )
        )
      );
    }
  })(state);
};

const handle_keydown = function _(state, key) {
  return (function temp_143(temp_142) {
    if (temp_142[0] === "state") {
      const time = temp_142[1].time;
      const window = temp_142[1].window;
      const old_left_paddle_y = temp_142[1].left_paddle_y;
      const old_right_paddle_y = temp_142[1].right_paddle_y;
      const ball_center_x = temp_142[1].ball_center_x;
      const ball_center_y = temp_142[1].ball_center_y;
      const ball_vx = temp_142[1].ball_vx;
      const ball_vy = temp_142[1].ball_vy;
      const old_left_paddle_vy_sign = temp_142[1].left_paddle_vy_sign;
      const old_right_paddle_vy_sign = temp_142[1].right_paddle_vy_sign;
      return (function temp_145(temp_144) {
        if (temp_144[0] === "true_") {
          return State_state({
            left_paddle_vy_sign: Opt_some(Sign, Sign_neg),
            left_paddle_y: (function temp_147(temp_146) {
              if (temp_146[0] === "true_") {
                return old_left_paddle_y;
              }
              if (temp_146[0] === "false_") {
                return clamp_paddle_y(
                  sub(
                    Int_nat(locy_raw(old_left_paddle_y)),
                    Int_nat(locy_raw(paddle_height))
                  )
                );
              }
            })(opt_sign_eq_some(old_left_paddle_vy_sign, Sign_neg)),
            time: time,
            window: window,
            right_paddle_y: old_right_paddle_y,
            ball_center_x: ball_center_x,
            ball_center_y: ball_center_y,
            ball_vx: ball_vx,
            ball_vy: ball_vy,
            right_paddle_vy_sign: old_right_paddle_vy_sign,
          });
        }
        if (temp_144[0] === "false_") {
          return (function temp_149(temp_148) {
            if (temp_148[0] === "true_") {
              return State_state({
                left_paddle_vy_sign: Opt_some(Sign, Sign_pos),
                left_paddle_y: (function temp_14b(temp_14a) {
                  if (temp_14a[0] === "true_") {
                    return old_left_paddle_y;
                  }
                  if (temp_14a[0] === "false_") {
                    return clamp_paddle_y(
                      add3(
                        Int_nat(locy_raw(old_left_paddle_y)),
                        Int_nat(locy_raw(paddle_height))
                      )
                    );
                  }
                })(opt_sign_eq_some(old_left_paddle_vy_sign, Sign_pos)),
                time: time,
                window: window,
                right_paddle_y: old_right_paddle_y,
                ball_center_x: ball_center_x,
                ball_center_y: ball_center_y,
                ball_vx: ball_vx,
                ball_vy: ball_vy,
                right_paddle_vy_sign: old_right_paddle_vy_sign,
              });
            }
            if (temp_148[0] === "false_") {
              return (function temp_14d(temp_14c) {
                if (temp_14c[0] === "true_") {
                  return State_state({
                    right_paddle_vy_sign: Opt_some(Sign, Sign_neg),
                    right_paddle_y: (function temp_14f(temp_14e) {
                      if (temp_14e[0] === "true_") {
                        return old_right_paddle_y;
                      }
                      if (temp_14e[0] === "false_") {
                        return clamp_paddle_y(
                          sub(
                            Int_nat(locy_raw(old_right_paddle_y)),
                            Int_nat(locy_raw(paddle_height))
                          )
                        );
                      }
                    })(opt_sign_eq_some(old_right_paddle_vy_sign, Sign_neg)),
                    time: time,
                    window: window,
                    left_paddle_y: old_left_paddle_y,
                    ball_center_x: ball_center_x,
                    ball_center_y: ball_center_y,
                    ball_vx: ball_vx,
                    ball_vy: ball_vy,
                    left_paddle_vy_sign: old_left_paddle_vy_sign,
                  });
                }
                if (temp_14c[0] === "false_") {
                  return (function temp_151(temp_150) {
                    if (temp_150[0] === "true_") {
                      return State_state({
                        right_paddle_vy_sign: Opt_some(Sign, Sign_pos),
                        right_paddle_y: (function temp_153(temp_152) {
                          if (temp_152[0] === "true_") {
                            return old_right_paddle_y;
                          }
                          if (temp_152[0] === "false_") {
                            return clamp_paddle_y(
                              add3(
                                Int_nat(locy_raw(old_right_paddle_y)),
                                Int_nat(locy_raw(paddle_height))
                              )
                            );
                          }
                        })(
                          opt_sign_eq_some(old_right_paddle_vy_sign, Sign_pos)
                        ),
                        time: time,
                        window: window,
                        left_paddle_y: old_left_paddle_y,
                        ball_center_x: ball_center_x,
                        ball_center_y: ball_center_y,
                        ball_vx: ball_vx,
                        ball_vy: ball_vy,
                        left_paddle_vy_sign: old_left_paddle_vy_sign,
                      });
                    }
                    if (temp_150[0] === "false_") {
                      return (function temp_155(temp_154) {
                        if (temp_154[0] === "true_") {
                          return (function temp_157(temp_156) {
                            if (temp_156[0] === "true_") {
                              return State_state({
                                ball_vx: ball_initial_vx,
                                ball_vy: ball_initial_vy,
                                window: window,
                                time: time,
                                ball_center_x: ball_center_x,
                                ball_center_y: ball_center_y,
                                left_paddle_y: old_left_paddle_y,
                                right_paddle_y: old_right_paddle_y,
                                left_paddle_vy_sign: old_left_paddle_vy_sign,
                                right_paddle_vy_sign: old_right_paddle_vy_sign,
                              });
                            }
                            if (temp_156[0] === "false_") {
                              return state;
                            }
                          })(is_ball_stationary_at_center(state));
                        }
                        if (temp_154[0] === "false_") {
                          return state;
                        }
                      })(str_list_contains(launch_ball_strs, key));
                    }
                  })(str_list_contains(right_paddle_down_strs, key));
                }
              })(str_list_contains(right_paddle_up_strs, key));
            }
          })(str_list_contains(left_paddle_down_strs, key));
        }
      })(str_list_contains(left_paddle_up_strs, key));
    }
  })(state);
};

const handle_keyup = function _(state, key) {
  return (function temp_159(temp_158) {
    if (temp_158[0] === "state") {
      const time = temp_158[1].time;
      const window = temp_158[1].window;
      const left_paddle_y = temp_158[1].left_paddle_y;
      const right_paddle_y = temp_158[1].right_paddle_y;
      const ball_center_x = temp_158[1].ball_center_x;
      const ball_center_y = temp_158[1].ball_center_y;
      const ball_vx = temp_158[1].ball_vx;
      const ball_vy = temp_158[1].ball_vy;
      const left_paddle_vy_sign = temp_158[1].left_paddle_vy_sign;
      const right_paddle_vy_sign = temp_158[1].right_paddle_vy_sign;
      return (function temp_15b(temp_15a) {
        if (temp_15a[0] === "true_") {
          return State_state({
            left_paddle_vy_sign: (function temp_15d(temp_15c) {
              if (temp_15c[0] === "none") {
                const _2 = temp_15c[1];
                return left_paddle_vy_sign;
              }
              if (temp_15c[0] === "some") {
                const _2 = temp_15c[1];
                const some_left_paddle_vy_sign = temp_15c[2];
                return (function temp_15f(temp_15e) {
                  if (temp_15e[0] === "neg") {
                    return Opt_none(Sign);
                  }
                  if (temp_15e[0] === "pos") {
                    return left_paddle_vy_sign;
                  }
                })(ascribe(Sign, some_left_paddle_vy_sign));
              }
            })(left_paddle_vy_sign),
            time: time,
            window: window,
            left_paddle_y: left_paddle_y,
            right_paddle_y: right_paddle_y,
            ball_center_x: ball_center_x,
            ball_center_y: ball_center_y,
            ball_vx: ball_vx,
            ball_vy: ball_vy,
            right_paddle_vy_sign: right_paddle_vy_sign,
          });
        }
        if (temp_15a[0] === "false_") {
          return (function temp_161(temp_160) {
            if (temp_160[0] === "true_") {
              return State_state({
                left_paddle_vy_sign: (function temp_163(temp_162) {
                  if (temp_162[0] === "none") {
                    const _2 = temp_162[1];
                    return left_paddle_vy_sign;
                  }
                  if (temp_162[0] === "some") {
                    const _2 = temp_162[1];
                    const some_left_paddle_vy_sign = temp_162[2];
                    return (function temp_165(temp_164) {
                      if (temp_164[0] === "pos") {
                        return Opt_none(Sign);
                      }
                      if (temp_164[0] === "neg") {
                        return left_paddle_vy_sign;
                      }
                    })(ascribe(Sign, some_left_paddle_vy_sign));
                  }
                })(left_paddle_vy_sign),
                time: time,
                window: window,
                left_paddle_y: left_paddle_y,
                right_paddle_y: right_paddle_y,
                ball_center_x: ball_center_x,
                ball_center_y: ball_center_y,
                ball_vx: ball_vx,
                ball_vy: ball_vy,
                right_paddle_vy_sign: right_paddle_vy_sign,
              });
            }
            if (temp_160[0] === "false_") {
              return (function temp_167(temp_166) {
                if (temp_166[0] === "true_") {
                  return State_state({
                    right_paddle_vy_sign: (function temp_169(temp_168) {
                      if (temp_168[0] === "none") {
                        const _2 = temp_168[1];
                        return right_paddle_vy_sign;
                      }
                      if (temp_168[0] === "some") {
                        const _2 = temp_168[1];
                        const some_right_paddle_vy_sign = temp_168[2];
                        return (function temp_16b(temp_16a) {
                          if (temp_16a[0] === "neg") {
                            return Opt_none(Sign);
                          }
                          if (temp_16a[0] === "pos") {
                            return right_paddle_vy_sign;
                          }
                        })(ascribe(Sign, some_right_paddle_vy_sign));
                      }
                    })(right_paddle_vy_sign),
                    time: time,
                    window: window,
                    left_paddle_y: left_paddle_y,
                    right_paddle_y: right_paddle_y,
                    ball_center_x: ball_center_x,
                    ball_center_y: ball_center_y,
                    ball_vx: ball_vx,
                    ball_vy: ball_vy,
                    left_paddle_vy_sign: left_paddle_vy_sign,
                  });
                }
                if (temp_166[0] === "false_") {
                  return (function temp_16d(temp_16c) {
                    if (temp_16c[0] === "true_") {
                      return State_state({
                        right_paddle_vy_sign: (function temp_16f(temp_16e) {
                          if (temp_16e[0] === "none") {
                            const _2 = temp_16e[1];
                            return right_paddle_vy_sign;
                          }
                          if (temp_16e[0] === "some") {
                            const _2 = temp_16e[1];
                            const some_right_paddle_vy_sign = temp_16e[2];
                            return (function temp_171(temp_170) {
                              if (temp_170[0] === "pos") {
                                return Opt_none(Sign);
                              }
                              if (temp_170[0] === "neg") {
                                return right_paddle_vy_sign;
                              }
                            })(ascribe(Sign, some_right_paddle_vy_sign));
                          }
                        })(right_paddle_vy_sign),
                        time: time,
                        window: window,
                        left_paddle_y: left_paddle_y,
                        right_paddle_y: right_paddle_y,
                        ball_center_x: ball_center_x,
                        ball_center_y: ball_center_y,
                        ball_vx: ball_vx,
                        ball_vy: ball_vy,
                        left_paddle_vy_sign: left_paddle_vy_sign,
                      });
                    }
                    if (temp_16c[0] === "false_") {
                      return state;
                    }
                  })(str_list_contains(right_paddle_down_strs, key));
                }
              })(str_list_contains(right_paddle_up_strs, key));
            }
          })(str_list_contains(left_paddle_down_strs, key));
        }
      })(str_list_contains(left_paddle_up_strs, key));
    }
  })(state);
};

const handle = function _(state, event) {
  return (function temp_173(temp_172) {
    if (temp_172[0] === "window_resize") {
      const new_window = temp_172[1];
      return handle_window_resize(state, new_window);
    }
    if (temp_172[0] === "keyup") {
      const key = temp_172[1];
      return handle_keyup(state, key);
    }
    if (temp_172[0] === "keydown") {
      const key = temp_172[1];
      return handle_keydown(state, key);
    }
  })(event);
};

export const app = App_app(State, render, tick, init, handle);

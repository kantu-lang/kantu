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
  return (function temp_d(temp_c) {
    if (temp_c[0] === "zero") {
      return Bool_true_;
    }
    if (temp_c[0] === "one") {
      return Bool_false_;
    }
  })(a);
};

const is_one = function _(a) {
  return (function temp_f(temp_e) {
    if (temp_e[0] === "zero") {
      return Bool_false_;
    }
    if (temp_e[0] === "one") {
      return Bool_true_;
    }
  })(a);
};

const eq2 = function _(a, b) {
  return (function temp_11(temp_10) {
    if (temp_10[0] === "zero") {
      return is_zero(b);
    }
    if (temp_10[0] === "one") {
      return is_one(b);
    }
  })(a);
};

const neq2 = function _(a, b) {
  return not(eq2(a, b));
};

const eq3 = function eq3(a, b) {
  return (function temp_13(temp_12) {
    if (temp_12[0] === "one") {
      return (function temp_15(temp_14) {
        if (temp_14[0] === "one") {
          return Bool_true_;
        }
        if (temp_14[0] === "extend") {
          const _ = temp_14[1];
          const _2 = temp_14[2];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_12[0] === "extend") {
      const a_left = temp_12[1];
      const a_right = temp_12[2];
      return (function temp_17(temp_16) {
        if (temp_16[0] === "one") {
          return Bool_false_;
        }
        if (temp_16[0] === "extend") {
          const b_left = temp_16[1];
          const b_right = temp_16[2];
          return and(eq3(a_left, b_left), eq2(a_right, b_right));
        }
      })(b);
    }
  })(a);
};

const neq3 = function _(a, b) {
  return not(eq3(a, b));
};

const minimal_bitlist_plus = function minimal_bitlist_plus(
  a,
  right_accumulator
) {
  return (function temp_19(temp_18) {
    if (temp_18[0] === "one") {
      return List_cons(Bit, Bit_one, right_accumulator);
    }
    if (temp_18[0] === "extend") {
      const a_left = temp_18[1];
      const a_right = temp_18[2];
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
  return (function temp_1b(temp_1a) {
    if (temp_1a[0] === "one") {
      return Pos_extend(Pos_one, Bit_zero);
    }
    if (temp_1a[0] === "extend") {
      const a_left = temp_1a[1];
      const a_right = temp_1a[2];
      return (function temp_1d(temp_1c) {
        if (temp_1c[0] === "zero") {
          return Pos_extend(a_left, Bit_one);
        }
        if (temp_1c[0] === "one") {
          return Pos_extend(succ(a_left), Bit_zero);
        }
      })(a_right);
    }
  })(a);
};

const pred = function pred(a) {
  return (function temp_1f(temp_1e) {
    if (temp_1e[0] === "one") {
      return Nat_zero;
    }
    if (temp_1e[0] === "extend") {
      const a_left = temp_1e[1];
      const a_right = temp_1e[2];
      return Nat_pos(
        (function temp_21(temp_20) {
          if (temp_20[0] === "one") {
            return Pos_extend(a_left, Bit_zero);
          }
          if (temp_20[0] === "zero") {
            return (function temp_23(temp_22) {
              if (temp_22[0] === "zero") {
                return Pos_one;
              }
              if (temp_22[0] === "pos") {
                const a_left_pred = temp_22[1];
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
  return (function temp_25(temp_24) {
    if (temp_24[0] === "one") {
      return Bit_one;
    }
    if (temp_24[0] === "extend") {
      const _2 = temp_24[1];
      const right = temp_24[2];
      return right;
    }
  })(a);
};

const neg = Int_neg;

const add = function add(a, b) {
  return (function temp_27(temp_26) {
    if (temp_26[0] === "one") {
      return succ(b);
    }
    if (temp_26[0] === "extend") {
      const a_left = temp_26[1];
      const a_right = temp_26[2];
      return (function temp_29(temp_28) {
        if (temp_28[0] === "one") {
          return succ(a);
        }
        if (temp_28[0] === "extend") {
          const b_left = temp_28[1];
          const b_right = temp_28[2];
          return (function temp_2b(temp_2a) {
            if (temp_2a[0] === "zero") {
              return Pos_extend(add(a_left, b_left), b_right);
            }
            if (temp_2a[0] === "one") {
              return (function temp_2d(temp_2c) {
                if (temp_2c[0] === "zero") {
                  return Pos_extend(add(a_left, b_left), Bit_one);
                }
                if (temp_2c[0] === "one") {
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
  return (function temp_2f(temp_2e) {
    if (temp_2e[0] === "one") {
      return b;
    }
    if (temp_2e[0] === "extend") {
      const a_left = temp_2e[1];
      const a_right = temp_2e[2];
      return (function temp_31(temp_30) {
        if (temp_30[0] === "zero") {
          return Pos_extend(mul(a_left, b), Bit_zero);
        }
        if (temp_30[0] === "one") {
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
  return (function temp_33(temp_32) {
    if (temp_32[0] === "one") {
      return a;
    }
    if (temp_32[0] === "extend") {
      const b_left = temp_32[1];
      const b_right = temp_32[2];
      return (function temp_35(temp_34) {
        if (temp_34[0] === "zero") {
          return square(pow(a, b_left));
        }
        if (temp_34[0] === "one") {
          return mul(a, square(pow(a, b_left)));
        }
      })(b_right);
    }
  })(b);
};

const le = function le(a, b) {
  return (function temp_37(temp_36) {
    if (temp_36[0] === "one") {
      return Bool_true_;
    }
    if (temp_36[0] === "extend") {
      const a_left = temp_36[1];
      const a_right = temp_36[2];
      return (function temp_39(temp_38) {
        if (temp_38[0] === "one") {
          return Bool_false_;
        }
        if (temp_38[0] === "extend") {
          const b_left = temp_38[1];
          const b_right = temp_38[2];
          return (function temp_3b(temp_3a) {
            if (temp_3a[0] === "zero") {
              return le(a_left, b_left);
            }
            if (temp_3a[0] === "one") {
              return (function temp_3d(temp_3c) {
                if (temp_3c[0] === "one") {
                  return le(a_left, b_left);
                }
                if (temp_3c[0] === "zero") {
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
  return (function temp_3f(temp_3e) {
    if (temp_3e[0] === "true_") {
      return Ord_lt;
    }
    if (temp_3e[0] === "false_") {
      return (function temp_41(temp_40) {
        if (temp_40[0] === "true_") {
          return Ord_eq;
        }
        if (temp_40[0] === "false_") {
          return Ord_gt;
        }
      })(eq3(a, b));
    }
  })(lt(a, b));
};

const min = function _(a, b) {
  return (function temp_43(temp_42) {
    if (temp_42[0] === "true_") {
      return a;
    }
    if (temp_42[0] === "false_") {
      return b;
    }
  })(lt(a, b));
};

const max = function _(a, b) {
  return (function temp_45(temp_44) {
    if (temp_44[0] === "true_") {
      return a;
    }
    if (temp_44[0] === "false_") {
      return b;
    }
  })(gt(a, b));
};

const one = Nat_pos(Pos_one);

const eq4 = function eq4(a, b) {
  return (function temp_47(temp_46) {
    if (temp_46[0] === "zero") {
      return (function temp_49(temp_48) {
        if (temp_48[0] === "zero") {
          return Bool_true_;
        }
        if (temp_48[0] === "pos") {
          const _ = temp_48[1];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_46[0] === "pos") {
      const ap = temp_46[1];
      return (function temp_4b(temp_4a) {
        if (temp_4a[0] === "zero") {
          return Bool_false_;
        }
        if (temp_4a[0] === "pos") {
          const bp = temp_4a[1];
          return eq3(ap, bp);
        }
      })(b);
    }
  })(a);
};

const neq4 = function _(a, b) {
  return not(eq4(a, b));
};

const succ2 = function succ2(a) {
  return (function temp_4d(temp_4c) {
    if (temp_4c[0] === "zero") {
      return one;
    }
    if (temp_4c[0] === "pos") {
      const ap = temp_4c[1];
      return Nat_pos(succ(ap));
    }
  })(a);
};

const pred2 = function pred2(a) {
  return (function temp_4f(temp_4e) {
    if (temp_4e[0] === "zero") {
      return Int_neg(Pos_one);
    }
    if (temp_4e[0] === "pos") {
      const ap = temp_4e[1];
      return Int_nat(pred(ap));
    }
  })(a);
};

const from_bit = function _(a) {
  return (function temp_51(temp_50) {
    if (temp_50[0] === "zero") {
      return Nat_zero;
    }
    if (temp_50[0] === "one") {
      return one;
    }
  })(a);
};

const extend_right = function _(a, right) {
  return (function temp_53(temp_52) {
    if (temp_52[0] === "zero") {
      return from_bit(right);
    }
    if (temp_52[0] === "pos") {
      const ap = temp_52[1];
      return Nat_pos(Pos_extend(ap, right));
    }
  })(a);
};

const extend_right_with_bits = function extend_right_with_bits(a, right) {
  return (function temp_55(temp_54) {
    if (temp_54[0] === "nil") {
      const _ = temp_54[1];
      return a;
    }
    if (temp_54[0] === "cons") {
      const _ = temp_54[1];
      const car = temp_54[2];
      const cdr = temp_54[3];
      return extend_right_with_bits(extend_right(a, car), cdr);
    }
  })(right);
};

const from_bitlist = function _(bits) {
  return extend_right_with_bits(Nat_zero, bits);
};

const parity2 = function _(a) {
  return (function temp_57(temp_56) {
    if (temp_56[0] === "zero") {
      return Bit_zero;
    }
    if (temp_56[0] === "pos") {
      const ap = temp_56[1];
      return parity(ap);
    }
  })(a);
};

const neg2 = function _(a) {
  return (function temp_59(temp_58) {
    if (temp_58[0] === "zero") {
      return Int_nat(Nat_zero);
    }
    if (temp_58[0] === "pos") {
      const ap = temp_58[1];
      return Int_neg(ap);
    }
  })(a);
};

const add2 = function add2(a, b) {
  return (function temp_5b(temp_5a) {
    if (temp_5a[0] === "zero") {
      return b;
    }
    if (temp_5a[0] === "pos") {
      const ap = temp_5a[1];
      return (function temp_5d(temp_5c) {
        if (temp_5c[0] === "zero") {
          return a;
        }
        if (temp_5c[0] === "pos") {
          const bp = temp_5c[1];
          return Nat_pos(add(ap, bp));
        }
      })(b);
    }
  })(a);
};

const mul2 = function mul2(a, b) {
  return (function temp_5f(temp_5e) {
    if (temp_5e[0] === "zero") {
      return Nat_zero;
    }
    if (temp_5e[0] === "pos") {
      const ap = temp_5e[1];
      return (function temp_61(temp_60) {
        if (temp_60[0] === "zero") {
          return Nat_zero;
        }
        if (temp_60[0] === "pos") {
          const bp = temp_60[1];
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
  return (function temp_63(temp_62) {
    if (temp_62[0] === "zero") {
      return one;
    }
    if (temp_62[0] === "pos") {
      const bp = temp_62[1];
      return (function temp_65(temp_64) {
        if (temp_64[0] === "zero") {
          return Nat_zero;
        }
        if (temp_64[0] === "pos") {
          const ap = temp_64[1];
          return Nat_pos(pow(ap, bp));
        }
      })(a);
    }
  })(b);
};

const le2 = function le2(a, b) {
  return (function temp_67(temp_66) {
    if (temp_66[0] === "zero") {
      return Bool_true_;
    }
    if (temp_66[0] === "pos") {
      const ap = temp_66[1];
      return (function temp_69(temp_68) {
        if (temp_68[0] === "zero") {
          return Bool_false_;
        }
        if (temp_68[0] === "pos") {
          const bp = temp_68[1];
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
  return (function temp_6b(temp_6a) {
    if (temp_6a[0] === "true_") {
      return Ord_lt;
    }
    if (temp_6a[0] === "false_") {
      return (function temp_6d(temp_6c) {
        if (temp_6c[0] === "true_") {
          return Ord_eq;
        }
        if (temp_6c[0] === "false_") {
          return Ord_gt;
        }
      })(eq4(a, b));
    }
  })(lt2(a, b));
};

const min2 = function _(a, b) {
  return (function temp_6f(temp_6e) {
    if (temp_6e[0] === "true_") {
      return a;
    }
    if (temp_6e[0] === "false_") {
      return b;
    }
  })(lt2(a, b));
};

const max2 = function _(a, b) {
  return (function temp_71(temp_70) {
    if (temp_70[0] === "true_") {
      return a;
    }
    if (temp_70[0] === "false_") {
      return b;
    }
  })(gt2(a, b));
};

const one2 = Int_nat(Nat_pos(Pos_one));

const eq5 = function eq5(a, b) {
  return (function temp_73(temp_72) {
    if (temp_72[0] === "neg") {
      const neg_a = temp_72[1];
      return (function temp_75(temp_74) {
        if (temp_74[0] === "neg") {
          const neg_b = temp_74[1];
          return eq3(neg_a, neg_b);
        }
        if (temp_74[0] === "nat") {
          const _ = temp_74[1];
          return Bool_false_;
        }
      })(b);
    }
    if (temp_72[0] === "nat") {
      const an = temp_72[1];
      return (function temp_77(temp_76) {
        if (temp_76[0] === "neg") {
          const _ = temp_76[1];
          return Bool_false_;
        }
        if (temp_76[0] === "nat") {
          const bn = temp_76[1];
          return eq4(an, bn);
        }
      })(b);
    }
  })(a);
};

const neq5 = function _(a, b) {
  return not(eq5(a, b));
};

const succ3 = function _(a) {
  return (function temp_79(temp_78) {
    if (temp_78[0] === "neg") {
      const neg_a = temp_78[1];
      return neg2(pred(neg_a));
    }
    if (temp_78[0] === "nat") {
      const an = temp_78[1];
      return Int_nat(succ2(an));
    }
  })(a);
};

const pred3 = function _(a) {
  return (function temp_7b(temp_7a) {
    if (temp_7a[0] === "neg") {
      const neg_a = temp_7a[1];
      return Int_neg(succ(neg_a));
    }
    if (temp_7a[0] === "nat") {
      const an = temp_7a[1];
      return pred2(an);
    }
  })(a);
};

const sign = function _(a) {
  return (function temp_7d(temp_7c) {
    if (temp_7c[0] === "neg") {
      const _2 = temp_7c[1];
      return Opt_some(Sign, Sign_neg);
    }
    if (temp_7c[0] === "nat") {
      const an = temp_7c[1];
      return (function temp_7f(temp_7e) {
        if (temp_7e[0] === "zero") {
          return Opt_none(Sign);
        }
        if (temp_7e[0] === "pos") {
          const _2 = temp_7e[1];
          return Opt_some(Sign, Sign_pos);
        }
      })(an);
    }
  })(a);
};

const parity3 = function _(a) {
  return (function temp_81(temp_80) {
    if (temp_80[0] === "neg") {
      const neg_a = temp_80[1];
      return parity(neg_a);
    }
    if (temp_80[0] === "nat") {
      const an = temp_80[1];
      return parity2(an);
    }
  })(a);
};

const neg3 = function _(a) {
  return (function temp_83(temp_82) {
    if (temp_82[0] === "neg") {
      const neg_a = temp_82[1];
      return Int_nat(Nat_pos(neg_a));
    }
    if (temp_82[0] === "nat") {
      const an = temp_82[1];
      return neg2(an);
    }
  })(a);
};

const double_ = function _(a) {
  return (function temp_85(temp_84) {
    if (temp_84[0] === "neg") {
      const neg_a = temp_84[1];
      return Int_neg(Pos_extend(neg_a, Bit_zero));
    }
    if (temp_84[0] === "nat") {
      const an = temp_84[1];
      return (function temp_87(temp_86) {
        if (temp_86[0] === "zero") {
          return a;
        }
        if (temp_86[0] === "pos") {
          const ap = temp_86[1];
          return Int_nat(Nat_pos(Pos_extend(ap, Bit_zero)));
        }
      })(an);
    }
  })(a);
};

const sub_pos = function sub_pos(a, b) {
  return (function temp_89(temp_88) {
    if (temp_88[0] === "one") {
      return Int_nat(pred(a));
    }
    if (temp_88[0] === "extend") {
      const b_left = temp_88[1];
      const b_right = temp_88[2];
      return (function temp_8b(temp_8a) {
        if (temp_8a[0] === "one") {
          return neg2(pred(b));
        }
        if (temp_8a[0] === "extend") {
          const a_left = temp_8a[1];
          const a_right = temp_8a[2];
          return (function temp_8d(temp_8c) {
            if (temp_8c[0] === "zero") {
              return (function temp_8f(temp_8e) {
                if (temp_8e[0] === "zero") {
                  return double_(sub_pos(a_left, b_left));
                }
                if (temp_8e[0] === "one") {
                  return succ3(double_(sub_pos(a_left, b_left)));
                }
              })(a_right);
            }
            if (temp_8c[0] === "one") {
              return (function temp_91(temp_90) {
                if (temp_90[0] === "one") {
                  return double_(sub_pos(a_left, b_left));
                }
                if (temp_90[0] === "zero") {
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
  return (function temp_93(temp_92) {
    if (temp_92[0] === "neg") {
      const neg_a = temp_92[1];
      return (function temp_95(temp_94) {
        if (temp_94[0] === "neg") {
          const neg_b = temp_94[1];
          return Int_neg(add(neg_a, neg_b));
        }
        if (temp_94[0] === "nat") {
          const bn = temp_94[1];
          return (function temp_97(temp_96) {
            if (temp_96[0] === "zero") {
              return a;
            }
            if (temp_96[0] === "pos") {
              const bp = temp_96[1];
              return sub_pos(bp, neg_a);
            }
          })(bn);
        }
      })(b);
    }
    if (temp_92[0] === "nat") {
      const an = temp_92[1];
      return (function temp_99(temp_98) {
        if (temp_98[0] === "zero") {
          return b;
        }
        if (temp_98[0] === "pos") {
          const ap = temp_98[1];
          return (function temp_9b(temp_9a) {
            if (temp_9a[0] === "neg") {
              const neg_b = temp_9a[1];
              return sub_pos(ap, neg_b);
            }
            if (temp_9a[0] === "nat") {
              const bn = temp_9a[1];
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
  return (function temp_9d(temp_9c) {
    if (temp_9c[0] === "neg") {
      const neg_a = temp_9c[1];
      return (function temp_9f(temp_9e) {
        if (temp_9e[0] === "neg") {
          const neg_b = temp_9e[1];
          return Int_nat(Nat_pos(mul(neg_a, neg_b)));
        }
        if (temp_9e[0] === "nat") {
          const bn = temp_9e[1];
          return neg2(mul2(Nat_pos(neg_a), bn));
        }
      })(b);
    }
    if (temp_9c[0] === "nat") {
      const an = temp_9c[1];
      return (function temp_a1(temp_a0) {
        if (temp_a0[0] === "neg") {
          const neg_b = temp_a0[1];
          return neg2(mul2(an, Nat_pos(neg_b)));
        }
        if (temp_a0[0] === "nat") {
          const bn = temp_a0[1];
          return Int_nat(mul2(an, bn));
        }
      })(b);
    }
  })(a);
};

const floor_div_bitlist = function floor_div_bitlist(
  dividend_left,
  dividend_right,
  divisor
) {
  return (function temp_a3(temp_a2) {
    if (temp_a2[0] === "nil") {
      const _ = temp_a2[1];
      return List_nil(Bit);
    }
    if (temp_a2[0] === "cons") {
      const _ = temp_a2[1];
      const car = temp_a2[2];
      const cdr = temp_a2[3];
      return (function temp_a5(temp_a4) {
        if (temp_a4[0] === "neg") {
          const _2 = temp_a4[1];
          return List_cons(
            Bit,
            Bit_zero,
            floor_div_bitlist(extend_right(dividend_left, car), cdr, divisor)
          );
        }
        if (temp_a4[0] === "nat") {
          const remainder = temp_a4[1];
          return List_cons(
            Bit,
            Bit_one,
            floor_div_bitlist(remainder, cdr, divisor)
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

const floor_div_pos = function _(a, b) {
  return from_bitlist(floor_div_bitlist(Nat_zero, minimal_bitlist(a), b));
};

const floor_div_nat = function _(a, b) {
  return (function temp_a7(temp_a6) {
    if (temp_a6[0] === "zero") {
      return Nat_zero;
    }
    if (temp_a6[0] === "pos") {
      const ap = temp_a6[1];
      return floor_div_pos(ap, b);
    }
  })(a);
};

const floor_div = function _(a, b) {
  return (function temp_a9(temp_a8) {
    if (temp_a8[0] === "neg") {
      const neg_a = temp_a8[1];
      return neg2(floor_div_pos(neg_a, b));
    }
    if (temp_a8[0] === "nat") {
      const an = temp_a8[1];
      return Int_nat(floor_div_nat(an, b));
    }
  })(a);
};

const floor_div_signed_divisor = function _(a, b_mag, b_sign) {
  return (function temp_ab(temp_aa) {
    if (temp_aa[0] === "pos") {
      return floor_div(a, b_mag);
    }
    if (temp_aa[0] === "neg") {
      return neg3(floor_div(a, b_mag));
    }
  })(b_sign);
};

const square3 = function _(a) {
  return mul3(a, a);
};

const pow3 = function _(a, b) {
  return (function temp_ad(temp_ac) {
    if (temp_ac[0] === "zero") {
      return one2;
    }
    if (temp_ac[0] === "pos") {
      const bp = temp_ac[1];
      return (function temp_af(temp_ae) {
        if (temp_ae[0] === "neg") {
          const neg_a = temp_ae[1];
          return (function temp_b1(temp_b0) {
            if (temp_b0[0] === "zero") {
              return Int_nat(Nat_pos(pow(neg_a, bp)));
            }
            if (temp_b0[0] === "one") {
              return Int_neg(pow(neg_a, bp));
            }
          })(parity(bp));
        }
        if (temp_ae[0] === "nat") {
          const an = temp_ae[1];
          return Int_nat(pow2(an, b));
        }
      })(a);
    }
  })(b);
};

const le3 = function _(a, b) {
  return (function temp_b3(temp_b2) {
    if (temp_b2[0] === "neg") {
      const neg_a = temp_b2[1];
      return (function temp_b5(temp_b4) {
        if (temp_b4[0] === "nat") {
          const _2 = temp_b4[1];
          return Bool_true_;
        }
        if (temp_b4[0] === "neg") {
          const neg_b = temp_b4[1];
          return ge(neg_a, neg_b);
        }
      })(b);
    }
    if (temp_b2[0] === "nat") {
      const an = temp_b2[1];
      return (function temp_b7(temp_b6) {
        if (temp_b6[0] === "neg") {
          const _2 = temp_b6[1];
          return Bool_false_;
        }
        if (temp_b6[0] === "nat") {
          const bn = temp_b6[1];
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
  return (function temp_b9(temp_b8) {
    if (temp_b8[0] === "true_") {
      return Ord_lt;
    }
    if (temp_b8[0] === "false_") {
      return (function temp_bb(temp_ba) {
        if (temp_ba[0] === "true_") {
          return Ord_eq;
        }
        if (temp_ba[0] === "false_") {
          return Ord_gt;
        }
      })(eq5(a, b));
    }
  })(lt3(a, b));
};

const min3 = function _(a, b) {
  return (function temp_bd(temp_bc) {
    if (temp_bc[0] === "true_") {
      return a;
    }
    if (temp_bc[0] === "false_") {
      return b;
    }
  })(lt3(a, b));
};

const max3 = function _(a, b) {
  return (function temp_bf(temp_be) {
    if (temp_be[0] === "true_") {
      return a;
    }
    if (temp_be[0] === "false_") {
      return b;
    }
  })(gt3(a, b));
};

const nat_2 = add2(one, one);

const nat_3 = add2(nat_2, one);

const nat_8 = pow2(nat_2, nat_3);

const nat_2pow8 = pow2(nat_2, nat_8);

const U8 = { type_species: "U8", type_args: [] };

const U8_u8 = function U8_u8(n, upper) {
  return ["u8", n, upper];
};

const Time = { type_species: "Time", type_args: [] };

const Time_time = function Time_time(millis_since_epoch) {
  return ["time", millis_since_epoch];
};

const String = { type_species: "String", type_args: [] };

const String_utf8 = function String_utf8(bytes) {
  return ["utf8", bytes];
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

const relu = function _(n) {
  return (function temp_c1(temp_c0) {
    if (temp_c0[0] === "neg") {
      const _2 = temp_c0[1];
      return Nat_zero;
    }
    if (temp_c0[0] === "nat") {
      const nn = temp_c0[1];
      return nn;
    }
  })(n);
};

const Pfrac = { type_species: "Pfrac", type_args: [] };

const Pfrac_pfrac = function Pfrac_pfrac(num, div) {
  return ["pfrac", num, div];
};

const pfrac_mult = function _(pf, right) {
  return (function temp_c3(temp_c2) {
    if (temp_c2[0] === "pfrac") {
      const num = temp_c2[1];
      const div = temp_c2[2];
      return floor_div_nat(mul2(Nat_pos(num), right), div);
    }
  })(pf);
};

const pos1 = Pos_one;

const pos2 = add(pos1, pos1);

const pos3 = add(pos2, pos1);

const pos4 = add(pos3, pos1);

const pos8 = pow(pos2, pos3);

const pos64 = mul(pos8, pos8);

const pos65 = add(pos1, pos64);

const pos66 = add(pos1, pos65);

const pos67 = add(pos1, pos66);

const refl_true = Equal_refl(Bool, Bool_true_);

const u8_65 = U8_u8(Nat_pos(pos65), refl_true);

const u8_66 = U8_u8(Nat_pos(pos66), refl_true);

const u8_67 = U8_u8(Nat_pos(pos67), refl_true);

const half = Pfrac_pfrac(pos1, pos2);

const paddle_height_factor = Pfrac_pfrac(pos1, pos8);

const ball_height_factor = Pfrac_pfrac(pos1, pos64);

const paddle_x_margin_factor = Pfrac_pfrac(pos1, pos64);

const paddle_width_factor = Pfrac_pfrac(pos1, pos64);

const background_image_str = String_utf8(List_cons(U8, u8_65, List_nil(U8)));

const paddle_image_str = String_utf8(List_cons(U8, u8_66, List_nil(U8)));

const ball_image_str = String_utf8(List_cons(U8, u8_67, List_nil(U8)));

const State = { type_species: "State", type_args: [] };

const State_state = function State_state({
  time: time,
  window: window,
  left_paddle_y: left_paddle_y,
  right_paddle_y: right_paddle_y,
  ball_x: ball_x,
  ball_y: ball_y,
  ball_vx: ball_vx,
  ball_vy: ball_vy,
}) {
  return [
    "state",
    {
      time: time,
      window: window,
      left_paddle_y: left_paddle_y,
      right_paddle_y: right_paddle_y,
      ball_x: ball_x,
      ball_y: ball_y,
      ball_vx: ball_vx,
      ball_vy: ball_vy,
    },
  ];
};

const init = function _(window, time) {
  return (function temp_c5(temp_c4) {
    if (temp_c4[0] === "window") {
      const window_w = temp_c4[1];
      const window_h = temp_c4[2];
      return State_state({
        time: time,
        window: window,
        left_paddle_y: relu(
          sub(
            Int_nat(pfrac_mult(half, window_h)),
            Int_nat(
              pfrac_mult(half, pfrac_mult(paddle_height_factor, window_h))
            )
          )
        ),
        right_paddle_y: relu(
          sub(
            Int_nat(pfrac_mult(half, window_h)),
            Int_nat(
              pfrac_mult(half, pfrac_mult(paddle_height_factor, window_h))
            )
          )
        ),
        ball_x: relu(
          sub(
            Int_nat(pfrac_mult(half, window_w)),
            Int_nat(pfrac_mult(half, pfrac_mult(ball_height_factor, window_h)))
          )
        ),
        ball_y: relu(
          sub(
            Int_nat(pfrac_mult(half, window_h)),
            Int_nat(pfrac_mult(half, pfrac_mult(ball_height_factor, window_h)))
          )
        ),
        ball_vx: Sign_pos,
        ball_vy: Sign_neg,
      });
    }
  })(window);
};

const render_background = function _(state) {
  return (function temp_c7(temp_c6) {
    if (temp_c6[0] === "state") {
      const window = temp_c6[1].window;
      return (function temp_c9(temp_c8) {
        if (temp_c8[0] === "window") {
          const window_w = temp_c8[1];
          const window_h = temp_c8[2];
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
  return (function temp_cb(temp_ca) {
    if (temp_ca[0] === "state") {
      const window = temp_ca[1].window;
      const left_paddle_y = temp_ca[1].left_paddle_y;
      return (function temp_cd(temp_cc) {
        if (temp_cc[0] === "window") {
          const window_w = temp_cc[1];
          const window_h = temp_cc[2];
          return Entity_scaled(
            pfrac_mult(paddle_x_margin_factor, window_w),
            left_paddle_y,
            pfrac_mult(paddle_width_factor, window_w),
            pfrac_mult(paddle_height_factor, window_h),
            paddle_image_str
          );
        }
      })(window);
    }
  })(state);
};

const render_right_paddle = function _(state) {
  return (function temp_cf(temp_ce) {
    if (temp_ce[0] === "state") {
      const window = temp_ce[1].window;
      const right_paddle_y = temp_ce[1].right_paddle_y;
      return (function temp_d1(temp_d0) {
        if (temp_d0[0] === "window") {
          const window_w = temp_d0[1];
          const window_h = temp_d0[2];
          return Entity_scaled(
            relu(
              sub(
                sub(
                  Int_nat(window_w),
                  Int_nat(pfrac_mult(paddle_x_margin_factor, window_w))
                ),
                Int_nat(pfrac_mult(paddle_width_factor, window_w))
              )
            ),
            right_paddle_y,
            pfrac_mult(paddle_width_factor, window_w),
            pfrac_mult(paddle_height_factor, window_h),
            paddle_image_str
          );
        }
      })(window);
    }
  })(state);
};

const render_ball = function _(state) {
  return (function temp_d3(temp_d2) {
    if (temp_d2[0] === "state") {
      const window = temp_d2[1].window;
      const ball_x = temp_d2[1].ball_x;
      const ball_y = temp_d2[1].ball_y;
      return (function temp_d5(temp_d4) {
        if (temp_d4[0] === "window") {
          const window_w = temp_d4[1];
          const window_h = temp_d4[2];
          return Entity_scaled(
            ball_x,
            ball_y,
            pfrac_mult(ball_height_factor, window_h),
            pfrac_mult(ball_height_factor, window_h),
            ball_image_str
          );
        }
      })(window);
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

const tick = function _(state, time) {
  return state;
};

const handle = function _(state, event) {
  return state;
};

export const app = App_app(State, render, tick, init, handle);

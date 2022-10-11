use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub struct RegisterSet {
	p: u64, // program counter
	q: u64, // used for return values
	r: u64, // used for return values
	s: u64, // stack pointer
	t: u64,
	u: u64,
	v: u64,
	w: u64,

	a: u64,
	b: u64,
	c: u64,
	d: u64,
	e: u64,
	f: u64,
	x: u64,
	y: u64,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum VmNativeRegister {
	P = 0b0000,
	Q = 0b0001,
	R = 0b0010,
	S = 0b0011,
	T = 0b0100,
	U = 0b0101,
	V = 0b0110,
	W = 0b0111,
	X = 0b1000,
	Y = 0b1001,
	A = 0xa,
	B = 0xb,
	C = 0xc,
	D = 0xd,
	E = 0xe,
	F = 0xf,
	Sink,
}

impl Debug for VmNativeRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use VmNativeRegister::*;

		let name = match self {
			P => "p",
			Q => "q",
			R => "r",
			S => "s",
			T => "t",
			U => "u",
			V => "v",
			W => "w",
			X => "x",
			Y => "y",
			A => "a",
			B => "b",
			C => "c",
			D => "d",
			E => "e",
			F => "f",
			Sink => "Sink",
		};

		write!(f, "{}", name)
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmByteRegister(VmNativeRegister, u64);

impl Debug for VmByteRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", VmRegister::from(self))
	}
}

impl From<VmByteRegister> for VmRegister {
	fn from(reg: VmByteRegister) -> Self {
		VmRegister::Byte(reg.0, reg.1)
	}
}

impl From<&VmByteRegister> for VmRegister {
	fn from(reg: &VmByteRegister) -> Self {
		VmRegister::Byte(reg.0, reg.1)
	}
}

impl From<VmRegister> for VmByteRegister {
	fn from(reg: VmRegister) -> Self {
		match reg {
			VmRegister::Byte(r, i) => VmByteRegister(r, i),
			other => panic!("cannot convert {:?} to byte register", other),
		}
	}
}

impl From<&VmRegister> for VmByteRegister {
	fn from(reg: &VmRegister) -> Self {
		match reg {
			VmRegister::Byte(r, i) => VmByteRegister(*r, *i),
			other => panic!("cannot convert {:?} to byte register", other),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmQwordRegister(VmNativeRegister, u64);

impl Debug for VmQwordRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", VmRegister::from(self))
	}
}

impl From<VmQwordRegister> for VmRegister {
	fn from(reg: VmQwordRegister) -> Self {
		VmRegister::Qword(reg.0, reg.1)
	}
}

impl From<&VmQwordRegister> for VmRegister {
	fn from(reg: &VmQwordRegister) -> Self {
		VmRegister::Qword(reg.0, reg.1)
	}
}

impl From<VmRegister> for VmQwordRegister {
	fn from(reg: VmRegister) -> Self {
		match reg {
			VmRegister::Qword(r, i) => VmQwordRegister(r, i),
			other => panic!("cannot convert {:?} to qword register", other),
		}
	}
}

impl From<&VmRegister> for VmQwordRegister {
	fn from(reg: &VmRegister) -> Self {
		match reg {
			VmRegister::Qword(r, i) => VmQwordRegister(*r, *i),
			other => panic!("cannot convert {:?} to qword register", other),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmHwordRegister(VmNativeRegister, u64);

impl Debug for VmHwordRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", VmRegister::from(self))
	}
}

impl From<VmHwordRegister> for VmRegister {
	fn from(reg: VmHwordRegister) -> Self {
		VmRegister::Hword(reg.0, reg.1)
	}
}

impl From<&VmHwordRegister> for VmRegister {
	fn from(reg: &VmHwordRegister) -> Self {
		VmRegister::Hword(reg.0, reg.1)
	}
}

impl From<VmRegister> for VmHwordRegister {
	fn from(reg: VmRegister) -> Self {
		match reg {
			VmRegister::Hword(r, i) => VmHwordRegister(r, i),
			other => panic!("cannot convert {:?} to hword register", other),
		}
	}
}

impl From<&VmRegister> for VmHwordRegister {
	fn from(reg: &VmRegister) -> Self {
		match reg {
			VmRegister::Hword(r, i) => VmHwordRegister(*r, *i),
			other => panic!("cannot convert {:?} to hword register", other),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmWordRegister(VmNativeRegister);

impl Debug for VmWordRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", VmRegister::from(self))
	}
}

impl From<VmWordRegister> for VmRegister {
	fn from(reg: VmWordRegister) -> Self {
		VmRegister::Word(reg.0)
	}
}

impl From<&VmWordRegister> for VmRegister {
	fn from(reg: &VmWordRegister) -> Self {
		VmRegister::Word(reg.0)
	}
}

impl From<VmRegister> for VmWordRegister {
	fn from(reg: VmRegister) -> Self {
		match reg {
			VmRegister::Word(r) => VmWordRegister(r),
			other => panic!("cannot convert {:?} to word register", other),
		}
	}
}

impl From<&VmRegister> for VmWordRegister {
	fn from(reg: &VmRegister) -> Self {
		match reg {
			VmRegister::Word(r) => VmWordRegister(*r),
			other => panic!("cannot convert {:?} to word register", other),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VmRegister {
	Byte(VmNativeRegister, u64),
	Qword(VmNativeRegister, u64),
	Hword(VmNativeRegister, u64),
	Word(VmNativeRegister),
}

impl VmRegister {
	pub const fn byte(native_reg: VmNativeRegister, i: u64) -> Self {
		Self::Byte(native_reg, i)
	}

	pub const fn qword(native_reg: VmNativeRegister, i: u64) -> Self {
		Self::Qword(native_reg, i)
	}

	pub const fn hword(native_reg: VmNativeRegister, i: u64) -> Self {
		Self::Hword(native_reg, i)
	}

	pub const fn word(native_reg: VmNativeRegister) -> Self {
		Self::Word(native_reg)
	}
}

impl Debug for VmRegister {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			VmRegister::Byte(reg, i) => write!(f, "{:?}b{}", reg, i),
			VmRegister::Qword(reg, i) => write!(f, "{:?}q{}", reg, i),
			VmRegister::Hword(reg, i) => write!(f, "{:?}h{}", reg, i),
			VmRegister::Word(reg) => write!(f, "{:?}", reg),
		}
	}
}

impl RegisterSet {
	pub fn get_native_register_value(&self, reg: VmNativeRegister) -> u64 {
		use VmNativeRegister::*;

		match reg {
			P => self.p,
			Q => self.q,
			R => self.r,
			S => self.s,
			T => self.t,
			U => self.u,
			V => self.v,
			W => self.w,
			X => self.x,
			Y => self.y,
			A => self.a,
			B => self.b,
			C => self.c,
			D => self.d,
			E => self.e,
			F => self.f,
			Sink => unreachable!("VmRegister::sink must only be used as a destination"),
		}
	}

	pub fn get_register_value(&self, reg: VmRegister) -> u64 {
		use VmRegister::*;

		match reg {
			Word(native_reg) => self.get_native_register_value(native_reg),
			Byte(native_reg, i) => (self.get_native_register_value(native_reg) >> (i * 8)) & 0xff,
			Qword(native_reg, i) => {
				(self.get_native_register_value(native_reg) >> (i * 16)) & 0xffff
			}
			Hword(native_reg, i) => {
				(self.get_native_register_value(native_reg) >> (i * 32)) & 0xffffffff
			}
		}
	}

	pub fn set_native_register_value(&mut self, reg: VmNativeRegister, val: u64) {
		use VmNativeRegister::*;

		match reg {
			P => self.p = val,
			Q => self.q = val,
			R => self.r = val,
			S => self.s = val,
			T => self.t = val,
			U => self.u = val,
			V => self.v = val,
			W => self.w = val,
			X => self.x = val,
			Y => self.y = val,
			A => self.a = val,
			B => self.b = val,
			C => self.c = val,
			D => self.d = val,
			E => self.e = val,
			F => self.f = val,
			Sink => (),
		};
	}

	pub fn set_register_value(&mut self, reg: VmRegister, val: u64) {
		use VmRegister::*;

		match reg {
			Word(native_reg) => self.set_native_register_value(native_reg, val),
			Byte(native_reg, i) => {
				let cleared_native_reg =
					self.get_native_register_value(native_reg) & !(0xff << (i * 8));
				let sanitized_val = (val & 0xff) << (i * 8);
				self.set_native_register_value(native_reg, cleared_native_reg | sanitized_val);
			}
			Qword(native_reg, i) => {
				let cleared_native_reg =
					self.get_native_register_value(native_reg) & !(0xffff << (i * 16));
				let sanitized_val = (val & 0xffff) << (i * 16);
				self.set_native_register_value(native_reg, cleared_native_reg | sanitized_val);
			}
			Hword(native_reg, i) => {
				let cleared_native_reg =
					self.get_native_register_value(native_reg) & !(0xffffffff << (i * 32));
				let sanitized_val = (val & 0xffffffff) << (i * 32);
				self.set_native_register_value(native_reg, cleared_native_reg | sanitized_val);
			}
		};
	}
}

#[allow(non_upper_case_globals)]
pub mod reg {
	use super::VmNativeRegister::*;
	use super::VmRegister;

	pub const pb0: VmRegister = VmRegister::byte(P, 0);
	pub const pb1: VmRegister = VmRegister::byte(P, 1);
	pub const pb2: VmRegister = VmRegister::byte(P, 2);
	pub const pb3: VmRegister = VmRegister::byte(P, 3);
	pub const pb4: VmRegister = VmRegister::byte(P, 4);
	pub const pb5: VmRegister = VmRegister::byte(P, 5);
	pub const pb6: VmRegister = VmRegister::byte(P, 6);
	pub const pb7: VmRegister = VmRegister::byte(P, 7);
	pub const qb0: VmRegister = VmRegister::byte(Q, 0);
	pub const qb1: VmRegister = VmRegister::byte(Q, 1);
	pub const qb2: VmRegister = VmRegister::byte(Q, 2);
	pub const qb3: VmRegister = VmRegister::byte(Q, 3);
	pub const qb4: VmRegister = VmRegister::byte(Q, 4);
	pub const qb5: VmRegister = VmRegister::byte(Q, 5);
	pub const qb6: VmRegister = VmRegister::byte(Q, 6);
	pub const qb7: VmRegister = VmRegister::byte(Q, 7);
	pub const rb0: VmRegister = VmRegister::byte(R, 0);
	pub const rb1: VmRegister = VmRegister::byte(R, 1);
	pub const rb2: VmRegister = VmRegister::byte(R, 2);
	pub const rb3: VmRegister = VmRegister::byte(R, 3);
	pub const rb4: VmRegister = VmRegister::byte(R, 4);
	pub const rb5: VmRegister = VmRegister::byte(R, 5);
	pub const rb6: VmRegister = VmRegister::byte(R, 6);
	pub const rb7: VmRegister = VmRegister::byte(R, 7);
	pub const sb0: VmRegister = VmRegister::byte(S, 0);
	pub const sb1: VmRegister = VmRegister::byte(S, 1);
	pub const sb2: VmRegister = VmRegister::byte(S, 2);
	pub const sb3: VmRegister = VmRegister::byte(S, 3);
	pub const sb4: VmRegister = VmRegister::byte(S, 4);
	pub const sb5: VmRegister = VmRegister::byte(S, 5);
	pub const sb6: VmRegister = VmRegister::byte(S, 6);
	pub const sb7: VmRegister = VmRegister::byte(S, 7);
	pub const tb0: VmRegister = VmRegister::byte(T, 0);
	pub const tb1: VmRegister = VmRegister::byte(T, 1);
	pub const tb2: VmRegister = VmRegister::byte(T, 2);
	pub const tb3: VmRegister = VmRegister::byte(T, 3);
	pub const tb4: VmRegister = VmRegister::byte(T, 4);
	pub const tb5: VmRegister = VmRegister::byte(T, 5);
	pub const tb6: VmRegister = VmRegister::byte(T, 6);
	pub const tb7: VmRegister = VmRegister::byte(T, 7);
	pub const ub0: VmRegister = VmRegister::byte(U, 0);
	pub const ub1: VmRegister = VmRegister::byte(U, 1);
	pub const ub2: VmRegister = VmRegister::byte(U, 2);
	pub const ub3: VmRegister = VmRegister::byte(U, 3);
	pub const ub4: VmRegister = VmRegister::byte(U, 4);
	pub const ub5: VmRegister = VmRegister::byte(U, 5);
	pub const ub6: VmRegister = VmRegister::byte(U, 6);
	pub const ub7: VmRegister = VmRegister::byte(U, 7);
	pub const vb0: VmRegister = VmRegister::byte(V, 0);
	pub const vb1: VmRegister = VmRegister::byte(V, 1);
	pub const vb2: VmRegister = VmRegister::byte(V, 2);
	pub const vb3: VmRegister = VmRegister::byte(V, 3);
	pub const vb4: VmRegister = VmRegister::byte(V, 4);
	pub const vb5: VmRegister = VmRegister::byte(V, 5);
	pub const vb6: VmRegister = VmRegister::byte(V, 6);
	pub const vb7: VmRegister = VmRegister::byte(V, 7);
	pub const wb0: VmRegister = VmRegister::byte(W, 0);
	pub const wb1: VmRegister = VmRegister::byte(W, 1);
	pub const wb2: VmRegister = VmRegister::byte(W, 2);
	pub const wb3: VmRegister = VmRegister::byte(W, 3);
	pub const wb4: VmRegister = VmRegister::byte(W, 4);
	pub const wb5: VmRegister = VmRegister::byte(W, 5);
	pub const wb6: VmRegister = VmRegister::byte(W, 6);
	pub const wb7: VmRegister = VmRegister::byte(W, 7);
	pub const ab0: VmRegister = VmRegister::byte(A, 0);
	pub const ab1: VmRegister = VmRegister::byte(A, 1);
	pub const ab2: VmRegister = VmRegister::byte(A, 2);
	pub const ab3: VmRegister = VmRegister::byte(A, 3);
	pub const ab4: VmRegister = VmRegister::byte(A, 4);
	pub const ab5: VmRegister = VmRegister::byte(A, 5);
	pub const ab6: VmRegister = VmRegister::byte(A, 6);
	pub const ab7: VmRegister = VmRegister::byte(A, 7);
	pub const bb0: VmRegister = VmRegister::byte(B, 0);
	pub const bb1: VmRegister = VmRegister::byte(B, 1);
	pub const bb2: VmRegister = VmRegister::byte(B, 2);
	pub const bb3: VmRegister = VmRegister::byte(B, 3);
	pub const bb4: VmRegister = VmRegister::byte(B, 4);
	pub const bb5: VmRegister = VmRegister::byte(B, 5);
	pub const bb6: VmRegister = VmRegister::byte(B, 6);
	pub const bb7: VmRegister = VmRegister::byte(B, 7);
	pub const cb0: VmRegister = VmRegister::byte(C, 0);
	pub const cb1: VmRegister = VmRegister::byte(C, 1);
	pub const cb2: VmRegister = VmRegister::byte(C, 2);
	pub const cb3: VmRegister = VmRegister::byte(C, 3);
	pub const cb4: VmRegister = VmRegister::byte(C, 4);
	pub const cb5: VmRegister = VmRegister::byte(C, 5);
	pub const cb6: VmRegister = VmRegister::byte(C, 6);
	pub const cb7: VmRegister = VmRegister::byte(C, 7);
	pub const db0: VmRegister = VmRegister::byte(D, 0);
	pub const db1: VmRegister = VmRegister::byte(D, 1);
	pub const db2: VmRegister = VmRegister::byte(D, 2);
	pub const db3: VmRegister = VmRegister::byte(D, 3);
	pub const db4: VmRegister = VmRegister::byte(D, 4);
	pub const db5: VmRegister = VmRegister::byte(D, 5);
	pub const db6: VmRegister = VmRegister::byte(D, 6);
	pub const db7: VmRegister = VmRegister::byte(D, 7);
	pub const eb0: VmRegister = VmRegister::byte(E, 0);
	pub const eb1: VmRegister = VmRegister::byte(E, 1);
	pub const eb2: VmRegister = VmRegister::byte(E, 2);
	pub const eb3: VmRegister = VmRegister::byte(E, 3);
	pub const eb4: VmRegister = VmRegister::byte(E, 4);
	pub const eb5: VmRegister = VmRegister::byte(E, 5);
	pub const eb6: VmRegister = VmRegister::byte(E, 6);
	pub const eb7: VmRegister = VmRegister::byte(E, 7);
	pub const fb0: VmRegister = VmRegister::byte(F, 0);
	pub const fb1: VmRegister = VmRegister::byte(F, 1);
	pub const fb2: VmRegister = VmRegister::byte(F, 2);
	pub const fb3: VmRegister = VmRegister::byte(F, 3);
	pub const fb4: VmRegister = VmRegister::byte(F, 4);
	pub const fb5: VmRegister = VmRegister::byte(F, 5);
	pub const fb6: VmRegister = VmRegister::byte(F, 6);
	pub const fb7: VmRegister = VmRegister::byte(F, 7);
	pub const xb0: VmRegister = VmRegister::byte(X, 0);
	pub const xb1: VmRegister = VmRegister::byte(X, 1);
	pub const xb2: VmRegister = VmRegister::byte(X, 2);
	pub const xb3: VmRegister = VmRegister::byte(X, 3);
	pub const xb4: VmRegister = VmRegister::byte(X, 4);
	pub const xb5: VmRegister = VmRegister::byte(X, 5);
	pub const xb6: VmRegister = VmRegister::byte(X, 6);
	pub const xb7: VmRegister = VmRegister::byte(X, 7);
	pub const yb0: VmRegister = VmRegister::byte(Y, 0);
	pub const yb1: VmRegister = VmRegister::byte(Y, 1);
	pub const yb2: VmRegister = VmRegister::byte(Y, 2);
	pub const yb3: VmRegister = VmRegister::byte(Y, 3);
	pub const yb4: VmRegister = VmRegister::byte(Y, 4);
	pub const yb5: VmRegister = VmRegister::byte(Y, 5);
	pub const yb6: VmRegister = VmRegister::byte(Y, 6);
	pub const yb7: VmRegister = VmRegister::byte(Y, 7);

	pub const pq0: VmRegister = VmRegister::qword(P, 0);
	pub const pq1: VmRegister = VmRegister::qword(P, 1);
	pub const pq2: VmRegister = VmRegister::qword(P, 2);
	pub const pq3: VmRegister = VmRegister::qword(P, 3);
	pub const qq0: VmRegister = VmRegister::qword(Q, 0);
	pub const qq1: VmRegister = VmRegister::qword(Q, 1);
	pub const qq2: VmRegister = VmRegister::qword(Q, 2);
	pub const qq3: VmRegister = VmRegister::qword(Q, 3);
	pub const rq0: VmRegister = VmRegister::qword(R, 0);
	pub const rq1: VmRegister = VmRegister::qword(R, 1);
	pub const rq2: VmRegister = VmRegister::qword(R, 2);
	pub const rq3: VmRegister = VmRegister::qword(R, 3);
	pub const sq0: VmRegister = VmRegister::qword(S, 0);
	pub const sq1: VmRegister = VmRegister::qword(S, 1);
	pub const sq2: VmRegister = VmRegister::qword(S, 2);
	pub const sq3: VmRegister = VmRegister::qword(S, 3);
	pub const tq0: VmRegister = VmRegister::qword(T, 0);
	pub const tq1: VmRegister = VmRegister::qword(T, 1);
	pub const tq2: VmRegister = VmRegister::qword(T, 2);
	pub const tq3: VmRegister = VmRegister::qword(T, 3);
	pub const uq0: VmRegister = VmRegister::qword(U, 0);
	pub const uq1: VmRegister = VmRegister::qword(U, 1);
	pub const uq2: VmRegister = VmRegister::qword(U, 2);
	pub const uq3: VmRegister = VmRegister::qword(U, 3);
	pub const vq0: VmRegister = VmRegister::qword(V, 0);
	pub const vq1: VmRegister = VmRegister::qword(V, 1);
	pub const vq2: VmRegister = VmRegister::qword(V, 2);
	pub const vq3: VmRegister = VmRegister::qword(V, 3);
	pub const wq0: VmRegister = VmRegister::qword(W, 0);
	pub const wq1: VmRegister = VmRegister::qword(W, 1);
	pub const wq2: VmRegister = VmRegister::qword(W, 2);
	pub const wq3: VmRegister = VmRegister::qword(W, 3);
	pub const aq0: VmRegister = VmRegister::qword(A, 0);
	pub const aq1: VmRegister = VmRegister::qword(A, 1);
	pub const aq2: VmRegister = VmRegister::qword(A, 2);
	pub const aq3: VmRegister = VmRegister::qword(A, 3);
	pub const bq0: VmRegister = VmRegister::qword(B, 0);
	pub const bq1: VmRegister = VmRegister::qword(B, 1);
	pub const bq2: VmRegister = VmRegister::qword(B, 2);
	pub const bq3: VmRegister = VmRegister::qword(B, 3);
	pub const cq0: VmRegister = VmRegister::qword(C, 0);
	pub const cq1: VmRegister = VmRegister::qword(C, 1);
	pub const cq2: VmRegister = VmRegister::qword(C, 2);
	pub const cq3: VmRegister = VmRegister::qword(C, 3);
	pub const dq0: VmRegister = VmRegister::qword(D, 0);
	pub const dq1: VmRegister = VmRegister::qword(D, 1);
	pub const dq2: VmRegister = VmRegister::qword(D, 2);
	pub const dq3: VmRegister = VmRegister::qword(D, 3);
	pub const eq0: VmRegister = VmRegister::qword(E, 0);
	pub const eq1: VmRegister = VmRegister::qword(E, 1);
	pub const eq2: VmRegister = VmRegister::qword(E, 2);
	pub const eq3: VmRegister = VmRegister::qword(E, 3);
	pub const fq0: VmRegister = VmRegister::qword(F, 0);
	pub const fq1: VmRegister = VmRegister::qword(F, 1);
	pub const fq2: VmRegister = VmRegister::qword(F, 2);
	pub const fq3: VmRegister = VmRegister::qword(F, 3);
	pub const xq0: VmRegister = VmRegister::qword(X, 0);
	pub const xq1: VmRegister = VmRegister::qword(X, 1);
	pub const xq2: VmRegister = VmRegister::qword(X, 2);
	pub const xq3: VmRegister = VmRegister::qword(X, 3);
	pub const yq0: VmRegister = VmRegister::qword(Y, 0);
	pub const yq1: VmRegister = VmRegister::qword(Y, 1);
	pub const yq2: VmRegister = VmRegister::qword(Y, 2);
	pub const yq3: VmRegister = VmRegister::qword(Y, 3);

	pub const ph0: VmRegister = VmRegister::hword(P, 0);
	pub const ph1: VmRegister = VmRegister::hword(P, 1);
	pub const qh0: VmRegister = VmRegister::hword(Q, 0);
	pub const qh1: VmRegister = VmRegister::hword(Q, 1);
	pub const rh0: VmRegister = VmRegister::hword(R, 0);
	pub const rh1: VmRegister = VmRegister::hword(R, 1);
	pub const sh0: VmRegister = VmRegister::hword(S, 0);
	pub const sh1: VmRegister = VmRegister::hword(S, 1);
	pub const th0: VmRegister = VmRegister::hword(T, 0);
	pub const th1: VmRegister = VmRegister::hword(T, 1);
	pub const uh0: VmRegister = VmRegister::hword(U, 0);
	pub const uh1: VmRegister = VmRegister::hword(U, 1);
	pub const vh0: VmRegister = VmRegister::hword(V, 0);
	pub const vh1: VmRegister = VmRegister::hword(V, 1);
	pub const wh0: VmRegister = VmRegister::hword(W, 0);
	pub const wh1: VmRegister = VmRegister::hword(W, 1);
	pub const ah0: VmRegister = VmRegister::hword(A, 0);
	pub const ah1: VmRegister = VmRegister::hword(A, 1);
	pub const bh0: VmRegister = VmRegister::hword(B, 0);
	pub const bh1: VmRegister = VmRegister::hword(B, 1);
	pub const ch0: VmRegister = VmRegister::hword(C, 0);
	pub const ch1: VmRegister = VmRegister::hword(C, 1);
	pub const dh0: VmRegister = VmRegister::hword(D, 0);
	pub const dh1: VmRegister = VmRegister::hword(D, 1);
	pub const eh0: VmRegister = VmRegister::hword(E, 0);
	pub const eh1: VmRegister = VmRegister::hword(E, 1);
	pub const fh0: VmRegister = VmRegister::hword(F, 0);
	pub const fh1: VmRegister = VmRegister::hword(F, 1);
	pub const xh0: VmRegister = VmRegister::hword(X, 0);
	pub const xh1: VmRegister = VmRegister::hword(X, 1);
	pub const yh0: VmRegister = VmRegister::hword(Y, 0);
	pub const yh1: VmRegister = VmRegister::hword(Y, 1);

	pub const p: VmRegister = VmRegister::word(P);
	pub const q: VmRegister = VmRegister::word(Q);
	pub const r: VmRegister = VmRegister::word(R);
	pub const s: VmRegister = VmRegister::word(S);
	pub const t: VmRegister = VmRegister::word(T);
	pub const u: VmRegister = VmRegister::word(U);
	pub const v: VmRegister = VmRegister::word(V);
	pub const w: VmRegister = VmRegister::word(W);
	pub const a: VmRegister = VmRegister::word(A);
	pub const b: VmRegister = VmRegister::word(B);
	pub const c: VmRegister = VmRegister::word(C);
	pub const d: VmRegister = VmRegister::word(D);
	pub const e: VmRegister = VmRegister::word(E);
	pub const f: VmRegister = VmRegister::word(F);
	pub const x: VmRegister = VmRegister::word(X);
	pub const y: VmRegister = VmRegister::word(Y);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::vm::Vm;

	#[test]
	fn register_names() {
		use VmNativeRegister::*;

		assert_eq!(format!("{:?}", VmRegister::byte(X, 0)), "xb0");
		assert_eq!(format!("{:?}", VmRegister::qword(X, 0)), "xq0");
		assert_eq!(format!("{:?}", VmRegister::hword(X, 0)), "xh0");
		assert_eq!(format!("{:?}", X), "x");
	}

	#[test]
	fn byte_registers() {
		use VmNativeRegister::*;

		let mut vm = Vm::default();
		vm.set_register_value(reg::x, 0x0706050403020100);

		for i in 0..8 {
			assert_eq!(vm.get_register_value(VmRegister::byte(X, i)), i);
			vm.set_register_value(VmRegister::byte(Y, i), i);
		}

		assert_eq!(vm.get_register_value(reg::y), 0x0706050403020100);

		vm.set_register_value(VmRegister::byte(Y, 4), 0xffff);
		assert_eq!(vm.get_register_value(reg::y), 0x070605ff03020100);
	}
}

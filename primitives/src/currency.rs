// This file is part of Setheum.

// Copyright (C) 2020-2021 Setheum Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![allow(clippy::from_over_into)] 

use bstringify::bstringify;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{
	convert::{Into, TryFrom},
	prelude::*,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

macro_rules! create_currency_id {
    ($(#[$meta:meta])*
	$vis:vis enum TokenSymbol {
        $($(#[$vmeta:meta])* $symbol:ident($name:expr, $deci:literal) = $val:literal,)*
    }) => {
		$(#[$meta])*
		$vis enum TokenSymbol {
			$($(#[$vmeta])* $symbol = $val,)*
		}

		impl TryFrom<u8> for TokenSymbol {
			type Error = ();

			fn try_from(v: u8) -> Result<Self, Self::Error> {
				match v {
					$($val => Ok(TokenSymbol::$symbol),)*
					_ => Err(()),
				}
			}
		}

		impl Into<u8> for TokenSymbol {
			fn into(self) -> u8 {
				match self {
					$(TokenSymbol::$symbol => ($val),)*
				}
			}
		}

		impl TryFrom<Vec<u8>> for CurrencyId {
			type Error = ();
			fn try_from(v: Vec<u8>) -> Result<CurrencyId, ()> {
				match v.as_slice() {
					$(bstringify!($symbol) => Ok(CurrencyId::Token(TokenSymbol::$symbol)),)*
					_ => Err(()),
				}
			}
		}
		impl TokenInfo for CurrencyId {
			fn currency_id(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($val),)*
					_ => None,
				}
			}
			fn name(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($name),)*
					_ => None,
				}
			}
			fn symbol(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some(stringify!($symbol)),)*
					_ => None,
				}
			}
			fn decimals(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($deci),)*
					_ => None,
				}
			}
		}

		$(pub const $symbol: CurrencyId = CurrencyId::Token(TokenSymbol::$symbol);)*
		
		impl TokenSymbol {
			pub fn get_info() -> Vec<(&'static str, u32)> {
				vec![
					$((stringify!($symbol), $deci),)*
				]
			}
		}
	}
}

create_currency_id! {
	// Represent a Token symbol with 8 bit
	// Bit 8 : 0 for Setheum Network, 1 for Neom Network
	// Bit 7 : Reserved
	// Bit 6 - 1 : The token ID
	#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[repr(u8)]
	pub enum TokenSymbol {
		/// Setheum Network >---------------------->>
		DNAR("Setheum Dinar", 10) = 0,
		SDEX("SettinDex", 10) = 1,
		SETT("Setter", 12) = 2,
		// SettCurrencies
		USDJ("Setheum US Dollar", 12) = 3,
		EURJ("Setheum Euro", 12) = 4,
		JPYJ("Setheum Japanese Yen", 12) = 5,
		GBPJ("Setheum Pound Sterling", 12) = 6,
		CADJ("Setheum Canadian Dollar", 12) = 7,
		HKDJ("Setheum HK Dollar", 12) = 8,
		TWDJ("Setheum Taiwan Dollar", 12) = 9,
		BRLJ("Setheum Brazilian Real", 12) = 10,
		CHFJ("Setheum Swiss Franc", 12) = 11,
		RUBJ("Setheum Russian Rubble", 12) = 12,
		THBJ("Setheum Thai Baht", 12) = 13,
		MXNJ("Setheum Mexican Peso", 12) = 14,
 		SARJ("Setheum Saudi Riyal", 12) = 15,
 		SGDJ("Setheum Singapore Dollar", 12) = 16,
 		SEKJ("Setheum Swedish Krona", 12) = 17,
		MYRJ("Setheum Malaysian Ringgit", 12) = 18,
		IDRJ("Setheum Indonesian Rupiah", 12) = 19,
 		NGNJ("Setheum Naira", 12) = 20,
 		PKRJ("Setheum Pakistani Rupee", 12) = 21,
		AEDJ("Setheum Emirati Dirham", 12) = 22,
		NOKJ("Setheum Norwegian Krone", 12) = 23,
		ZARJ("Setheum S.African Rand", 12) = 24,
		CZKJ("Setheum Czech Koruna", 12) = 25,
		NZDJ("Setheum NZ Dollar ", 12) = 26,
		COPJ("Setheum Colombian Peso", 12) = 27,
		KWDJ("Setheum Kuwaiti Dinar", 12) = 28,
		CLPJ("Setheum Chilean Peso", 12) = 29,
		PHPJ("Setheum Philippine Peso", 12) = 30,
		HUFJ("Setheum Hungarian Forint", 12) = 31,
		JODJ("Setheum Jordanian Dinar", 12) = 32,
		TRYJ("Setheum Turkish Lira", 12) = 33,
 		AUDJ("Setheum Aussie Dollar", 12) = 34,
 		KESJ("Setheum Kenyan Shilling", 12) = 35,
 		BHDJ("Setheum Bahraini Dinar", 12) = 36,
		BWPJ("Setheum Botswanan Pula", 12) = 37,
		INRJ("Setheum Indian Rupee", 12) = 38,
 		KRWJ("Setheum S.Korean Won", 12) = 39,
 		SCRJ("Setheum Seychellois Rupee", 12) = 40,
		ZMWJ("Setheum Zambian Kwacha", 12) = 41,
		GHSJ("Setheum Ghanaian Cedi", 12) = 42,
		AOAJ("Setheum Angolan Kwanza", 12) = 43,
		DZDJ("Setheum Algerian Dinar", 12) = 44,
		ETBJ("Setheum Ethiopian Birr", 12) = 45,
		TZSJ("Setheum TZ Shilling", 12) = 46,
		CFAJ("Setheum CFA Franc", 12) = 47,
		AZNJ("Setheum Azerbaijani Manat", 12) = 48,
		PLNJ("Setheum Polish Zloty", 12) = 49,

		/// Neom Network >---------------------->>
		NEOM("Neom", 10) = 128,
		HALAL("HalalSwap", 10) = 129,
		NSETT("Neom Setter", 12) = 130,
		// SettCurrencies
		JUSD("Neom US Dollar", 12) = 131,
		JEUR("Neom Euro", 12) = 132,
		JJPY("Neom Japanese Yen", 12) = 133,
		JGBP("Neom Pound Sterling", 12) = 134,
		JCAD("Neom Canadian Dollar", 12) = 135,
		JHKD("Neom HK Dollar", 12) = 136,
		JTWD("Neom Taiwan Dollar", 12) = 137,
		JBRL("Neom Brazilian Real", 12) = 138,
		JCHF("Neom Swiss Franc", 12) = 139,
		JRUB("Neom Russian Rubble", 12) = 140,
		JTHB("Neom Thai Baht", 12) = 141,
		JMXN("Neom Mexican Peso", 12) = 142,
 		JSAR("Neom Saudi Riyal", 12) = 143,
 		JSGD("Neom Singapore Dollar", 12) = 144,
 		JSEK("Neom Swedish Krona", 12) = 145,
		JMYR("Neom Malaysian Ringgit", 12) = 146,
		JIDR("Neom Indonesian Rupiah", 12) = 147,
 		JNGN("Neom Naira", 12) = 148,
 		JPKR("Neom Pakistani Rupee", 12) = 149,
		JAED("Neom Emirati Dirham", 12) = 150,
		JNOK("Neom Norwegian Krone", 12) = 151,
		JZAR("Neom S.African Rand", 12) = 152,
		JCZK("Neom Czech Koruna", 12) = 153,
		JNZD("Neom NZ Dollar ", 12) = 154,
		JCOP("Neom Colombian Peso", 12) = 155,
		JKWD("Neom Kuwaiti Dinar", 12) = 156,
		JCLP("Neom Chilean Peso", 12) = 157,
		JPHP("Neom Philippine Peso", 12) = 158,
		JHUF("Neom Hungarian Forint", 12) = 159,
		JJOD("Neom Jordanian Dinar", 12) = 160,
		JTRY("Neom Turkish Lira", 12) = 161,
 		JAUD("Neom Aussie Dollar", 12) = 162,
 		JKES("Neom Kenyan Shilling", 12) = 163,
 		JBHD("Neom Bahraini Dinar", 12) = 164,
		JBWP("Neom Botswanan Pula", 12) = 165,
		JINR("Neom Indian Rupee", 12) = 166,
 		JKRW("Neom S.Korean Won", 12) = 167,
 		JSCR("Neom Seychellois Rupee", 12) = 168,
		JZMW("Neom Zambian Kwacha", 12) = 169,
		JGHS("Neom Ghanaian Cedi", 12) = 170,
		JAOA("Neom Angolan Kwanza", 12) = 171,
		JDZD("Neom Algerian Dinar", 12) = 172,
		JETB("Neom Ethiopian Birr", 12) = 173,
		JTZS("Neom TZ Shilling", 12) = 174,
		JCFA("Neom CFA Franc", 12) = 175,
		JAZN("Neom Azerbaijani Manat", 12) = 176,
		JPLN("Neom Polish Zloty", 12) = 177,
	}
}

pub trait TokenInfo {
	fn currency_id(&self) -> Option<u8>;
	fn name(&self) -> Option<&str>;
	fn symbol(&self) -> Option<&str>;
	fn decimals(&self) -> Option<u8>;
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DexShare {
	Token(TokenSymbol),
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	Token(TokenSymbol),
	DexShare(DexShare, DexShare),
}

impl CurrencyId {
	pub fn is_token_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Token(_))
	}

	pub fn is_dex_share_currency_id(&self) -> bool {
		matches!(self, CurrencyId::DexShare(_, _))
	}

	pub fn split_dex_share_currency_id(&self) -> Option<(Self, Self)> {
		match self {
			CurrencyId::DexShare(token_symbol_0, token_symbol_1) => {
				let symbol_0: CurrencyId = (*token_symbol_0).into();
				let symbol_1: CurrencyId = (*token_symbol_1).into();
				Some((symbol_0, symbol_1))
			}
			_ => None,
		}
	}

	pub fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self> {
		let token_symbol_0 = match currency_id_0 {
			CurrencyId::Token(symbol) => {
				DexShare::Token(symbol)
			}
			_ => return None,
		};
		let token_symbol_1 = match currency_id_1 {
			CurrencyId::Token(symbol) => {
				DexShare::Token(symbol)
			}
			_ => return None,
		};
		Some(CurrencyId::DexShare(token_symbol_0, token_symbol_1))
	}
}

impl From<DexShare> for u32 {
	fn from(val: DexShare) -> u32 {
		let mut bytes = [0u8; 4];
		match val {
			DexShare::Token(token) => {
				bytes[3] = token.into();
			}
		}
		u32::from_be_bytes(bytes)
	}
}

impl Into<CurrencyId> for DexShare {
	fn into(self) -> CurrencyId {
		match self {
			DexShare::Token(token) => CurrencyId::Token(token),
		}
	}
}

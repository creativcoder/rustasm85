/*pub enum _TYPE {
	_1_BYTE,
	_2_BYTE,
	_3_BYTE,
}

use _TYPE::*;
*/
pub const aci:(&'static str,u16) = ("aci",0xCE);
pub const adc_a:(&'static str,u16) = ("adc a",0x8F);
pub const adc_b:(&'static str,u16) = ("adc b",0x88);
pub const adc_c:(&'static str,u16) = ("adc c",0x89);
pub const adc_d:(&'static str,u16) = ("adc d",0x8A);
pub const adc_e:(&'static str,u16) = ("adc e",0x8B);
pub const adc_h:(&'static str,u16) = ("adc h",0x8C);
pub const adc_l:(&'static str,u16) = ("adc l",0x8D);
pub const adc_m:(&'static str,u16) = ("adc m",0x8e);
pub const add_a:(&'static str,u16) = ("add a",0x87);
pub const add_b:(&'static str,u16) = ("add b",0x80);
pub const add_c:(&'static str,u16) = ("add c",0x81);
pub const add_d:(&'static str,u16) = ("add d",0x82);
pub const add_e:(&'static str,u16) = ("add e",0x83);
pub const add_h:(&'static str,u16) = ("add h",0x84);
pub const add_l:(&'static str,u16) = ("add l",0x85);
pub const add_m:(&'static str,u8) = ("add m",0x86);
pub const adi:(&'static str,u8) = ("adi",0xC6);
pub const ana_a:(&'static str,u8) = ("ana a",0xA7);
pub const ana_b:(&'static str,u8) = ("ana b",0xA0);
pub const ana_c:(&'static str,u8) = ("ana c",0xA1);
pub const ana_d:(&'static str,u8) = ("ana d",0xA2);
pub const ana_e:(&'static str,u8) = ("ana e",0xA3);
pub const ana_h:(&'static str,u8) = ("ana h",0xA4);
pub const ana_l:(&'static str,u8) = ("ana l",0xA5);
pub const ana_m:(&'static str,u8) = ("ana m",0xA6);
pub const ani:(&'static str,u8) = ("ani",0xE6);
pub const call:(&'static str,u8) = ("call",0xCD);
pub const cc:(&'static str,u8) = ("cc",0xDC);
pub const cm:(&'static str,u8) = ("cm",0xFC);
pub const cma:(&'static str,u8) = ("cma",0x2F);
pub const cmc:(&'static str,u8) = ("cmc",0x3F);
pub const cmp_a:(&'static str,u8) = ("cmp a",0xBF);
pub const cmp_b:(&'static str,u8) = ("cmp b",0xB8);
pub const cmp_c:(&'static str,u8) = ("cmp c",0xB9);
pub const cmp_d:(&'static str,u8) = ("cmp d",0xBA);
pub const cmp_e:(&'static str,u8) = ("cmp e",0xBB);
pub const cmp_h:(&'static str,u8) = ("cmp h",0xBC);
pub const cmp_l:(&'static str,u8) = ("cmp l",0xBD);
pub const cmp_m:(&'static str,u8) = ("cmp m",0xBD);
pub const cnc:(&'static str,u8) = ("cnc",0xD4);
pub const cnz:(&'static str,u8) = ("cnz",0xC4);
pub const cp:(&'static str,u8) = ("cp",0xF4);
pub const cpe:(&'static str,u8) = ("cpe",0xEC);
pub const cpi:(&'static str,u8) = ("cpi",0xFE);
pub const cpo:(&'static str,u8) = ("cpo",0xE4);
pub const cz:(&'static str,u8) = ("cz",0xCC);
pub const daa:(&'static str,u8) = ("daa",0x27);
pub const dad_b:(&'static str,u8) = ("dad b",0x09);
pub const dad_d:(&'static str,u8) = ("dad d",0x19);
pub const dad_h:(&'static str,u8) = ("dad h",0x29);
pub const dad_sp:(&'static str,u8) = ("dad sp",0x39);
pub const dcr_a:(&'static str,u8) = ("dcr a",0x3D);
pub const dcr_b:(&'static str,u8) = ("dcr b",0x05);
pub const dcr_c:(&'static str,u8) = ("dcr c",0x0D);
pub const dcr_d:(&'static str,u8) = ("dcr d",0x15);
pub const dcr_e:(&'static str,u8) = ("dcr e",0x1D);
pub const dcr_h:(&'static str,u8) = ("dcr h",0x25);
pub const dcr_l:(&'static str,u8) = ("dcr l",0x2D);
pub const dcr_m:(&'static str,u8) = ("dcr m",0x35);
pub const dcx_b:(&'static str,u8) = ("dcx b",0x0B);
pub const dcx_d:(&'static str,u8) = ("dcx d",0x1B);
pub const dcx_h:(&'static str,u8) = ("dcx h",0x2B);
pub const dcx_sp:(&'static str,u8) = ("dcx sp",0x3B);
pub const di:(&'static str,u8) = ("di",0xF3);
pub const ei:(&'static str,u8) = ("ei",0xFB);
pub const hlt:(&'static str,u8) = ("hlt",0x76);
pub const _in:(&'static str,u8) = ("in",0xDB);
pub const inr_a:(&'static str,u8) = ("inr a",0x3C);
pub const inr_b:(&'static str,u8) = ("inr b",0x04);
pub const inr_c:(&'static str,u8) = ("inr c",0x0C);
pub const inr_d:(&'static str,u8) = ("inr d",0x14);
pub const inr_e:(&'static str,u8) = ("inr e",0x1C);
pub const inr_h:(&'static str,u8) = ("inr h",0x24);
pub const inr_l:(&'static str,u8) = ("inr l",0x2C);
pub const inr_m:(&'static str,u8) = ("inr m",0x34);
pub const inx_b:(&'static str,u8) = ("inx b",0x03);
pub const inx_d:(&'static str,u8) = ("inx d",0x13);
pub const inx_h:(&'static str,u8) = ("inx h",0x23);
pub const inx_sp:(&'static str,u8) = ("inx sp",0x33);
//jc_label
pub const jc:(&'static str,u8) = ("jc",0xDA);
pub const jm:(&'static str,u8) = ("jm",0xFA);
pub const jmp:(&'static str,u8) = ("jmp",0xC3);
pub const jnc:(&'static str,u8) = ("jnc",0xD2);
pub const jnz:(&'static str,u8) = ("jnz",0xC2);
pub const jp:(&'static str,u8) = ("jp",0xF2);
pub const jpe:(&'static str,u8) = ("jpe",0xEA);
pub const jpo:(&'static str,u8) = ("jpo",0xE2);
pub const jz:(&'static str,u8) = ("jz",0xCA);
pub const lda:(&'static str,u8) = ("lda",0x3A);
pub const ldax_b:(&'static str,u8) = ("ldax b",0x0A);
pub const ldax_d:(&'static str,u8) = ("ldax d",0x1A);
pub const lhld:(&'static str,u8) = ("lhld",0x2A);
pub const lxi_b:(&'static str,u8) = ("lxi b",0x01);
pub const lxi_d:(&'static str,u8) = ("lxi d",0x11);
pub const lxi_h:(&'static str,u8) = ("lxi h",0x21);
pub const lxi_sp:(&'static str,u8) = ("lxi sp",0x31);
pub const mov_aa:(&'static str,u8) = ("mov a,a",0x7F);
pub const mov_ab:(&'static str,u8) = ("mov a,b",0x78);
pub const mov_ac:(&'static str,u8) = ("mov a,c",0x79);
pub const mov_ad:(&'static str,u8) = ("mov a,d",0x7A);
pub const mov_ae:(&'static str,u8) = ("mov a,e",0x7B);
pub const mov_ah:(&'static str,u8) = ("mov a,h",0x7C);
pub const mov_al:(&'static str,u8) = ("mov a,l",0x7D);
pub const mov_am:(&'static str,u8) = ("mov a,m",0x7E);
pub const mov_ba:(&'static str,u8) = ("mov b,",0x47);
pub const mov_bb:(&'static str,u8) = ("mov b,",0x40);
pub const mov_bc:(&'static str,u8) = ("mov b,",0x41);
pub const mov_bd:(&'static str,u8) = ("mov b,",0x42);
pub const mov_be:(&'static str,u8) = ("mov b,",0x43);
pub const mov_bh:(&'static str,u8) = ("mov b,",0x44);
pub const mov_bl:(&'static str,u8) = ("mov b,",0x45);
pub const mov_bm:(&'static str,u8) = ("mov b,",0x46);
pub const mov_ca:(&'static str,u8) = ("mov c,",0x4F);
pub const mov_cb:(&'static str,u8) = ("mov c,",0x48);
pub const mov_cc:(&'static str,u8) = ("mov c,",0x49);
pub const mov_cd:(&'static str,u8) = ("mov c,",0x4A);
pub const mov_ce:(&'static str,u8) = ("mov c,",0x4B);
pub const mov_ch:(&'static str,u8) = ("mov c,",0x4C);
pub const mov_cl:(&'static str,u8) = ("mov c,",0x4D);
pub const mov_cm:(&'static str,u8) = ("mov c,",0x4E);
pub const mov_da:(&'static str,u8) = ("mov d,",0x57);
pub const mov_db:(&'static str,u8) = ("mov d,",0x50);
pub const mov_dc:(&'static str,u8) = ("mov d,",0x51);
pub const mov_dd:(&'static str,u8) = ("mov d,",0x52);
pub const mov_de:(&'static str,u8) = ("mov d,",0x53);
pub const mov_dh:(&'static str,u8) = ("mov d,",0x54);
pub const mov_dl:(&'static str,u8) = ("mov d,",0x55);
pub const mov_dm:(&'static str,u8) = ("mov d,",0x56);
pub const mov_ea:(&'static str,u8) = ("mov e,",0x5F);
pub const mov_eb:(&'static str,u8) = ("mov e,",0x58);
pub const mov_ec:(&'static str,u8) = ("mov e,",0x59);
pub const mov_ed:(&'static str,u8) = ("mov e,",0x5A);
pub const mov_ee:(&'static str,u8) = ("mov e,",0x5B);
pub const mov_eh:(&'static str,u8) = ("mov e,",0x5C);
pub const mov_el:(&'static str,u8) = ("mov e,",0x5D);
pub const mov_em:(&'static str,u8) = ("mov e,",0x5E);
pub const mov_ha:(&'static str,u8) = ("mov h,",0x67);
pub const mov_hb:(&'static str,u8) = ("mov h,",0x60);
pub const mov_hc:(&'static str,u8) = ("mov h,",0x61);
pub const mov_hd:(&'static str,u8) = ("mov h,",0x62);
pub const mov_he:(&'static str,u8) = ("mov h,",0x63);
pub const mov_hh:(&'static str,u8) = ("mov h,",0x64);
pub const mov_hl:(&'static str,u8) = ("mov h,",0x65);
pub const mov_hm:(&'static str,u8) = ("mov h,",0x66);
pub const mov_la:(&'static str,u8) = ("mov l,",0x6F);
pub const mov_lb:(&'static str,u8) = ("mov l,",0x68);
pub const mov_lc:(&'static str,u8) = ("mov l,",0x69);
pub const mov_ld:(&'static str,u8) = ("mov l,",0x6A);
pub const mov_le:(&'static str,u8) = ("mov l,",0x6B);
pub const mov_lh:(&'static str,u8) = ("mov l,",0x6C);
pub const mov_ll:(&'static str,u8) = ("mov l,",0x6D);
pub const mov_lm:(&'static str,u8) = ("mov l,",0x6E);
pub const mov_ma:(&'static str,u8) = ("mov m,",0x77);
pub const mov_mb:(&'static str,u8) = ("mov m,",0x70);
pub const mov_mc:(&'static str,u8) = ("mov m,",0x71);
pub const mov_md:(&'static str,u8) = ("mov m,",0x72);
pub const mov_me:(&'static str,u8) = ("mov m,",0x73);
pub const mov_mh:(&'static str,u8) = ("mov m,",0x74);
pub const mov_ml:(&'static str,u8) = ("mov m,",0x75);
pub const mvi_a:(&'static str,u8) = ("mvi a,",0x3E);
pub const mvi_b:(&'static str,u8) = ("mvi b,",0x06);
pub const mvi_c:(&'static str,u8) = ("mvi c,",0x0E);
pub const mvi_d:(&'static str,u8) = ("mvi d,",0x16);
pub const mvi_e:(&'static str,u8) = ("mvi e,",0x1E);
pub const mvi_h:(&'static str,u8) = ("mvi h,",0x26);
pub const mvi_l:(&'static str,u8) = ("mvi l,",0x2E);
pub const mvi_m:(&'static str,u8) = ("mvi m,",0x36);
pub const nop:(&'static str,u8) = ("nop",0x00);
pub const ora_a:(&'static str,u8) = ("ora a",0xB7);
pub const ora_b:(&'static str,u8) = ("ora b",0xB0);
pub const ora_c:(&'static str,u8) = ("ora c",0xB1);
pub const ora_d:(&'static str,u8) = ("ora d",0xB2);
pub const ora_e:(&'static str,u8) = ("ora e",0xB3);
pub const ora_h:(&'static str,u8) = ("ora h",0xB4);
pub const ora_l:(&'static str,u8) = ("ora l",0xB5);
pub const ora_m:(&'static str,u8) = ("ora m",0xB6);
pub const ori:(&'static str,u8) = ("ori",0xF6);
pub const out:(&'static str,u8) = ("out",0xD3);
pub const pchl:(&'static str,u8) = ("pchl",0xE9);
pub const pop_b:(&'static str,u8) = ("pop b",0xC1);
pub const pop_d:(&'static str,u8) = ("pop d",0xD1);
pub const pop_h:(&'static str,u8) = ("pop h",0xE1);
pub const pop_psw:(&'static str,u8) = ("pop psw",0xF1);
pub const push_b:(&'static str,u8) = ("push b",0xC5);
pub const push_d:(&'static str,u8) = ("push d",0xD5);
pub const push_h:(&'static str,u8) = ("push h",0xE5);
pub const push_psw:(&'static str,u8) = ("push psw",0xF5);
pub const ral:(&'static str,u8) = ("ral",0x17);
pub const rar:(&'static str,u8) = ("rar",0x1F);
pub const rc:(&'static str,u8) = ("rc",0xD8);
pub const ret:(&'static str,u8) = ("ret",0xC9);
pub const rim:(&'static str,u8) = ("rim",0x20);
pub const rlc:(&'static str,u8) = ("rlc",0x07);
pub const rm:(&'static str,u8) = ("rm",0xF8);
pub const rnc:(&'static str,u8) = ("rnc",0xD0);
pub const rnz:(&'static str,u8) = ("rnz",0xC0);
pub const rp:(&'static str,u8) = ("rp",0xF0);
pub const rpe:(&'static str,u8) = ("rpe",0xE8);
pub const rpo:(&'static str,u8) = ("rpo",0xE0);
pub const rrc:(&'static str,u8) = ("rrc",0x0F);
pub const rst_0:(&'static str,u8) = ("rst 0",0xC7);
pub const rst_1:(&'static str,u8) = ("rst 1",0xCF);
pub const rst_2:(&'static str,u8) = ("rst 2",0xD7);
pub const rst_3:(&'static str,u8) = ("rst 3",0xDF);
pub const rst_4:(&'static str,u8) = ("rst 4",0xE7);
pub const rst_5:(&'static str,u8) = ("rst 5",0xEF);
pub const rst_6:(&'static str,u8) = ("rst 6",0xF7);
pub const rst_7:(&'static str,u8) = ("rst 7",0xFF);
pub const rz:(&'static str,u8) = ("rz",0xC8);
pub const sbb_a:(&'static str,u8) = ("sbb a",0x9F);
pub const sbb_b:(&'static str,u8) = ("sbb b",0x98);
pub const sbb_c:(&'static str,u8) = ("sbb c",0x99);
pub const sbb_d:(&'static str,u8) = ("sbb d",0x9A);
pub const sbb_e:(&'static str,u8) = ("sbb e",0x9B);
pub const sbb_h:(&'static str,u8) = ("sbb h",0x9C);
pub const sbb_l:(&'static str,u8) = ("sbb l",0x9d);
pub const sbb_m:(&'static str,u8) = ("sbb m",0x9E);
pub const sbi:(&'static str,u8) = ("sbi",0xDE);
pub const shld:(&'static str,u8) = ("shld",0x22);
pub const sim:(&'static str,u8) = ("sim",0x30);
pub const sphl:(&'static str,u8) = ("sphl",0xF9);
pub const sta:(&'static str,u8) = ("sta",0x32);
pub const stax_b:(&'static str,u8) = ("stax b",0x02);
pub const stax_d:(&'static str,u8) = ("stax d",0x12);
pub const stc:(&'static str,u8) = ("stc",0x37);
pub const sub_a:(&'static str,u8) = ("sub a",0x97);
pub const sub_b:(&'static str,u8) = ("sub b",0x90);
pub const sub_c:(&'static str,u8) = ("sub c",0x91);
pub const sub_d:(&'static str,u8) = ("sub d",0x92);
pub const sub_e:(&'static str,u8) = ("sub e",0x93);
pub const sub_h:(&'static str,u8) = ("sub h",0x94);
pub const sub_l:(&'static str,u8) = ("sub l",0x95);
pub const sub_m:(&'static str,u8) = ("sub m",0x96);
pub const sui:(&'static str,u8) = ("sui",0xD6);
pub const xchg:(&'static str,u8) = ("xchg",0xEB);
pub const xra_a:(&'static str,u8) = ("xra a",0xAF);
pub const xra_b:(&'static str,u8) = ("xra b",0xA8);
pub const xra_c:(&'static str,u8) = ("xra c",0xA9);
pub const xra_d:(&'static str,u8) = ("xra d",0xAA);
pub const xra_e:(&'static str,u8) = ("xra e",0xAB);
pub const xra_h:(&'static str,u8) = ("xra h",0xAC);
pub const xra_l:(&'static str,u8) = ("xra l",0xAD);
pub const xra_m:(&'static str,u8) = ("xra m",0xAE);
pub const xri:(&'static str,u8) = ("xri",0xEE);
pub const xthl:(&'static str,u8) = ("xthl",0xE3);

pub const lxi_h:(&'static str,u8) = ("lxi h",21);

/*const /*1.*/ ACI Data CE /*2*/
const /*2.*/ ADC A 8F /*1*/
const /*3.*/ ADC B 88 /*1*/
const /*4.*/ ADC C 89 /*1*/
const /*5.*/ ADC D 8A /*1*/
const /*6.*/ ADC E 8B /*1*/
const /*7.*/ ADC H 8C /*1*/
const /*8.*/ ADC L 8D /*1*/
const /*9.*/ ADC M 8E /*1*/
const /*10.*/ ADD A 87 /*1*/
const /*11.*/ ADD B 80 /*1*/
const /*12.*/ ADD C 81 /*1*/
const /*13.*/ ADD D 82 /*1*/
const /*14.*/ ADD E 83 /*1*/
const /*15.*/ ADD H 84 /*1*/
const /*16.*/ ADD L 85 /*1*/
const /*17.*/ ADD M 86 /*1*/
const /*18.*/ ADI Data C6 /*2*/
const /*19.*/ ANA A A7 /*1*/
const /*20.*/ ANA B A0 /*1*/
const /*21.*/ ANA C A1 /*1*/
const /*22.*/ ANA D A2 /*1*/
const /*23.*/ ANA E A3 /*1*/
const /*24.*/ ANA H A4 /*1*/
const /*25.*/ ANA L A5 /*1*/
const /*26.*/ ANA M A6 /*1*/
const /*27.*/ ANI Data E6 /*2*/
const /*28.*/ CALL Label CD /*3*/
const /*29.*/ CC Label DC /*3*/
const /*30.*/ CM Label FC /*3*/
const /*31.*/ CMA 2F /*1*/
const /*32.*/ CMC 3F /*1*/
const /*33.*/ CMP A BF /*1*/
const /*34.*/ CMP B B8 /*1*/
const /*35.*/ CMP C B9 /*1*/
const /*36.*/ CMP D BA /*1*/
const /*37.*/ CMP E BB /*1*/
const /*38.*/ CMP H BC /*1*/
const /*39.*/ CMP L BD /*1*/
const /*40.*/ CMP M BD /*1*/
const /*41.*/ CNC Label D4 /*3*/
const /*42.*/ CNZ Label C4 /*3*/
const /*43.*/ CP Label F4 /*3*/
const /*44.*/ CPE Label EC /*3*/
const /*45.*/ CPI Data FE /*2*/
const /*46.*/ CPO Label E4 /*3*/
const /*47.*/ CZ Label CC /*3*/
const /*48.*/ DAA 27 /*1*/
const /*49.*/ DAD B 09 /*1*/
const /*50.*/ DAD D 19 /*1*/
const /*51.*/ DAD H 29 /*1*/
const /*52.*/ DAD SP 39 /*1*/
const /*53.*/ DCR A 3D /*1*/
const /*54.*/ DCR B 05 /*1*/
const /*55.*/ DCR C 0D /*1*/
const /*56.*/ DCR D 15 /*1*/
const /*57.*/ DCR E 1D /*1*/
const /*58.*/ DCR H 25 /*1*/
const /*59.*/ DCR L 2D /*1*/
const /*60.*/ DCR M 35 /*1*/
const /*61.*/ DCX B 0B /*1*/
const /*62.*/ DCX D 1B /*1*/
const /*63.*/ DCX H 2B /*1*/
const /*64.*/ DCX SP 3B /*1*/
const /*65.*/ DI F3 /*1*/
const /*66.*/ EI FB /*1*/
const /*67.*/ HLT 76 /*1*/
const /*68.*/ IN Port-address DB /*2*/
const /*69.*/ INR A 3C /*1*/
const /*70.*/ INR B 04 /*1*/
const /*71.*/ INR C 0C /*1*/
const /*72.*/ INR D 14 /*1*/
const /*73.*/ INR E 1C /*1*/
const /*74.*/ INR H 24 /*1*/
const /*75.*/ INR L 2C /*1*/
const /*76.*/ INR M 34 /*1*/
const /*77.*/ INX B 03 /*1*/
const /*78.*/ INX D 13 /*1*/
const /*79.*/ INX H 23 /*1*/
const /*80.*/ INX SP 33 /*1*/
const /*81.*/ JC Label DA /*3*/
const /*82.*/ JM Label FA /*3*/
const /*83.*/ JMP Label C3 /*3*/
const /*84.*/ JNC Label D2 /*3*/
const /*85.*/ JNZ Label C2 /*3*/
const /*86.*/ JP Label F2 /*3*/
const /*87.*/ JPE Label EA /*3*/
const /*88.*/ JPO Label E2 /*3*/
const /*89.*/ JZ Label CA /*3*/
const /*90.*/ LDA Address 3A /*3*/
const /*91.*/ LDAX B 0A /*1*/
const /*92.*/ LDAX D 1A /*1*/
const /*93.*/ LHLD Address 2A /*3*/
const /*94.*/ LXI B 01 /*3*/
const /*95.*/ LXI D 11 /*3*/
const /*96.*/ LXI H 21 /*3*/
const /*97.*/ LXI SP 31 /*3*/
const /*98.*/ MOV A, A 7F /*1*/
const /*99.*/ MOV A, B 78 /*1*/
const /*100.*/ MOV A, C 79 /*1*/
const /*101.*/ MOV A, D 7A /*1*/
const /*102.*/ MOV A, E 7B /*1*/
const /*103.*/ MOV A, H 7C /*1*/
const /*104.*/ MOV A, L 7D /*1*/
const /*105.*/ MOV A, M 7E /*1*/
const /*106.*/ MOV B, A 47 /*1*/
const /*107.*/ MOV B, B 40 /*1*/
const /*108.*/ MOV B, C 41 /*1*/
const /*109.*/ MOV B, D 42 /*1*/
const /*110.*/ MOV B, E 43 /*1*/
const /*111.*/ MOV B, H 44 /*1*/
const /*112.*/ MOV B, L 45 /*1*/
const /*113.*/ MOV B, M 46 /*1*/
const /*114.*/ MOV C, A 4F /*1*/
const /*115.*/ MOV C, B 48 /*1*/
const /*116.*/ MOV C, C 49 /*1*/
const /*117.*/ MOV C, D 4A /*1*/
const /*118.*/ MOV C, E 4B /*1*/
const /*119.*/ MOV C, H 4C /*1*/
const /*120.*/ MOV C, L 4D /*1*/
const /*121.*/ MOV C, M 4E /*1*/
const /*122.*/ MOV D, A 57 /*1*/
const /*123.*/ MOV D, B 50 /*1*/
const /*124.*/ MOV D, C 51 /*1*/
const /*125.*/ MOV D, D 52 /*1*/
const /*126.*/ MOV D, E 53 /*1*/
const /*127.*/ MOV D, H 54 /*1*/
const /*128.*/ MOV D, L 55 /*1*/
const /*129.*/ MOV D, M 56 /*1*/
const /*130.*/ MOV E, A 5F /*1*/
const /*131.*/ MOV E, B 58 /*1*/
const /*132.*/ MOV E, C 59 /*1*/
const /*133.*/ MOV E, D 5A /*1*/
const /*134.*/ MOV E, E 5B /*1*/
const /*135.*/ MOV E, H 5C /*1*/
const /*136.*/ MOV E, L 5D /*1*/
const /*137.*/ MOV E, M 5E /*1*/
const /*138.*/ MOV H, A 67 /*1*/
const /*139.*/ MOV H, B 60 /*1*/
const /*140.*/ MOV H, C 61 /*1*/
const /*141.*/ MOV H, D 62 /*1*/
const /*142.*/ MOV H, E 63 /*1*/
const /*143.*/ MOV H, H 64 /*1*/
const /*144.*/ MOV H, L 65 /*1*/
const /*145.*/ MOV H, M 66 /*1*/
const /*146.*/ MOV L, A 6F /*1*/
const /*147.*/ MOV L, B 68 /*1*/
const /*148.*/ MOV L, C 69 /*1*/
const /*149.*/ MOV L, D 6A /*1*/
const /*150.*/ MOV L, E 6B /*1*/
const /*151.*/ MOV L, H 6C /*1*/
const /*152.*/ MOV L, L 6D /*1*/
const /*153.*/ MOV L, M 6E /*1*/
const /*154.*/ MOV M, A 77 /*1*/
const /*155.*/ MOV M, B 70 /*1*/
const /*156.*/ MOV M, C 71 /*1*/
const /*157.*/ MOV M, D 72 /*1*/
const /*158.*/ MOV M, E 73 /*1*/
const /*159.*/ MOV M, H 74 /*1*/
const /*160.*/ MOV M, L 75 /*1*/
const /*161.*/ MVI A, Data 3E /*2*/
const /*162.*/ MVI B, Data 06 /*2*/
const /*163.*/ MVI C, Data 0E /*2*/
const /*164.*/ MVI D, Data 16 /*2*/
const /*165.*/ MVI E, Data 1E /*2*/
const /*166.*/ MVI H, Data 26 /*2*/
const /*167.*/ MVI L, Data 2E /*2*/
const /*168.*/ MVI M, Data 36 /*2*/
const /*169.*/ NOP 00 /*1*/
const /*170.*/ ORA A B7 /*1*/
const /*171.*/ ORA B B0 /*1*/
const /*172.*/ ORA C B1 /*1*/
const /*173.*/ ORA D B2 /*1*/
const /*174.*/ ORA E B3 /*1*/
const /*175.*/ ORA H B4 /*1*/
const /*176.*/ ORA L B5 /*1*/
const /*177.*/ ORA M B6 /*1*/
const /*178.*/ ORI Data F6 /*2*/
const /*179.*/ OUT Port-Address D3 /*2*/
const /*180.*/ PCHL E9 /*1*/
const /*181.*/ POP B C1 /*1*/
const /*182.*/ POP D D1 /*1*/
const /*183.*/ POP H E1 /*1*/
const /*184.*/ POP PSW F1 /*1*/
const /*185.*/ PUSH B C5 /*1*/
const /*186.*/ PUSH D D5 /*1*/
const /*187.*/ PUSH H E5 /*1*/
const /*188.*/ PUSH PSW F5 /*1*/
const /*189.*/ RAL 17 /*1*/
const /*190.*/ RAR 1F /*1*/
const /*191.*/ RC D8 /*1*/
const /*192.*/ RET C9 /*1*/
const /*193.*/ RIM 20 /*1*/
const /*194.*/ RLC 07 /*1*/
const /*195.*/ RM F8 /*1*/
const /*196.*/ RNC D0 /*1*/
const /*197.*/ RNZ C0 /*1*/
const /*198.*/ RP F0 /*1*/
const /*199.*/ RPE E8 /*1*/
const /*200.*/ RPO E0 /*1*/
const /*201.*/ RRC 0F /*1*/
const /*202.*/ RST 0 C7 /*1*/
const /*203.*/ RST 1 CF /*1*/
const /*204.*/ RST 2 D7 /*1*/
const /*205.*/ RST 3 DF /*1*/
const /*206.*/ RST 4 E7 /*1*/
const /*207.*/ RST 5 EF /*1*/
const /*208.*/ RST 6 F7 /*1*/
const /*209.*/ RST 7 FF /*1*/
const /*210.*/ RZ C8 /*1*/
const /*211.*/ SBB A 9F /*1*/
const /*212.*/ SBB B 98 /*1*/
const /*213.*/ SBB C 99 /*1*/
const /*214.*/ SBB D 9A /*1*/
const /*215.*/ SBB E 9B /*1*/
const /*216.*/ SBB H 9C /*1*/
const /*217.*/ SBB L 9D /*1*/
const /*218.*/ SBB M 9E /*1*/
const /*219.*/ SBI Data DE /*2*/
const /*220.*/ SHLD Address 22 /*3*/
const /*221.*/ SIM 30 /*1*/
const /*222.*/ SPHL F9 /*1*/
const /*223.*/ STA Address 32 /*3*/
const /*224.*/ STAX B 02 /*1*/
const /*225.*/ STAX D 12 /*1*/
const /*226.*/ STC 37 /*1*/
const /*227.*/ SUB A 97 /*1*/
const /*228.*/ SUB B 90 /*1*/
const /*229.*/ SUB C 91 /*1*/
const /*230.*/ SUB D 92 /*1*/
const /*231.*/ SUB E 93 /*1*/
const /*232.*/ SUB H 94 /*1*/
const /*233.*/ SUB L 95 /*1*/
const /*234.*/ SUB M 96 /*1*/
const /*235.*/ SUI Data D6 /*2*/
const /*236.*/ XCHG EB /*1*/
const /*237.*/ XRA A AF /*1*/
const /*238.*/ XRA B A8 /*1*/
const /*239.*/ XRA C A9 /*1*/
const /*240.*/ XRA D AA /*1*/
const /*241.*/ XRA E AB /*1*/
const /*242.*/ XRA H AC /*1*/
const /*243.*/ XRA L AD /*1*/
const /*244.*/ XRA M AE /*1*/
const /*245.*/ XRI Data EE /*2*/
const /*246.*/ XTHL E3 /*1*/


#[test]
fn test_opcodes() {
	println!("test suite for opcodes");
}*/
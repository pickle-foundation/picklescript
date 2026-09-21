Fn 0:2
Ident 3:7 text="main"
LParen 7:8
RParen 8:9
LBrace 10:11
Newline 11:12
Let 16:19
Ident 20:21 text="m"
Assign 22:23
LBrace 24:25
Number 25:27 text="10" suf="" isf=0 int=10
Colon 27:28
Str 29:34 T("ten")
Comma 34:35
Number 36:38 text="20" suf="" isf=0 int=20
Colon 38:39
Str 40:48 T("twenty")
Comma 48:49
Number 50:52 text="30" suf="" isf=0 int=30
Colon 52:53
Str 54:62 T("thirty")
RBrace 62:63
Newline 63:64
Var 68:71
Ident 72:75 text="tot"
Assign 76:77
Number 78:79 text="0" suf="" isf=0 int=0
Newline 79:80
For 84:87
LParen 88:89
LParen 89:90
Ident 90:91 text="k"
Comma 91:92
Ident 93:94 text="v"
RParen 94:95
In 96:98
Ident 99:100 text="m"
RParen 100:101
LBrace 102:103
Newline 103:104
Ident 112:115 text="tot"
PlusEq 116:118
Ident 119:120 text="k"
Newline 120:121
Ident 129:136 text="println"
LParen 136:137
Str 137:146 E[Ident 139:140 text="k"; Eof 141:141] T(":") E[Ident 143:144 text="v"; Eof 145:145]
RParen 146:147
Newline 147:148
RBrace 152:153
Newline 153:154
Ident 158:165 text="println"
LParen 165:166
Str 166:177 T("tot=") E[Ident 172:175 text="tot"; Eof 176:176]
RParen 177:178
Newline 178:179
Ident 183:184 text="m"
LBracket 184:185
Number 185:187 text="40" suf="" isf=0 int=40
RBracket 187:188
Assign 189:190
Str 191:198 T("forty")
Newline 198:199
Ident 203:210 text="println"
LParen 210:211
Str 211:230 T("has40=") E[Ident 219:220 text="m"; Dot 220:221; Ident 221:224 text="has"; LParen 224:225; Number 225:227 text="40" suf="" isf=0 int=40; RParen 227:228; Eof 229:229]
RParen 230:231
Newline 231:232
Ident 236:243 text="println"
LParen 243:244
Str 244:251 T("miss=")
Plus 252:253
LParen 254:255
Ident 255:256 text="m"
Dot 256:257
Get 257:260
LParen 260:261
Number 261:263 text="99" suf="" isf=0 int=99
RParen 263:264
QuestionQuestion 265:267
Str 268:271 T("-")
RParen 271:272
RParen 272:273
Newline 273:274
Let 278:281
Ident 282:284 text="mc"
Assign 285:286
LBrace 287:288
Char 288:291 ch='a' int=97
Colon 291:292
Number 293:295 text="97" suf="" isf=0 int=97
Comma 295:296
Char 297:300 ch='z' int=122
Colon 300:301
Number 302:305 text="122" suf="" isf=0 int=122
RBrace 305:306
Newline 306:307
Ident 311:318 text="println"
LParen 318:319
Str 319:344 T("a=") E[Ident 323:325 text="mc"; LBracket 325:326; Char 326:329 ch='a' int=97; RBracket 329:330; Eof 331:331] T(" z=") E[Ident 335:337 text="mc"; LBracket 337:338; Char 338:341 ch='z' int=122; RBracket 341:342; Eof 343:343]
RParen 344:345
Newline 345:346
RBrace 346:347
Eof 347:347

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
Str 25:28 T("a")
Colon 28:29
Number 30:31 text="1" suf="" isf=0 int=1
Comma 31:32
Str 33:36 T("b")
Colon 36:37
Number 38:39 text="2" suf="" isf=0 int=2
Comma 39:40
Str 41:44 T("c")
Colon 44:45
Number 46:47 text="3" suf="" isf=0 int=3
RBrace 47:48
Newline 48:49
Var 53:56
Ident 57:60 text="cat"
Assign 61:62
Str 63:65
Newline 65:66
Var 70:73
Ident 74:77 text="sum"
Assign 78:79
Number 80:81 text="0" suf="" isf=0 int=0
Newline 81:82
For 86:89
LParen 90:91
LParen 91:92
Ident 92:93 text="k"
Comma 93:94
Ident 95:96 text="v"
RParen 96:97
In 98:100
Ident 101:102 text="m"
RParen 102:103
LBrace 104:105
Newline 105:106
Ident 114:117 text="cat"
PlusEq 118:120
Ident 121:122 text="k"
Newline 122:123
Ident 131:134 text="sum"
PlusEq 135:137
Ident 138:139 text="v"
Newline 139:140
RBrace 144:145
Newline 145:146
Ident 150:157 text="println"
LParen 157:158
Str 158:183 T("entries ") E[Ident 168:171 text="cat"; Eof 172:172] T(" sum=") E[Ident 178:181 text="sum"; Eof 182:182]
RParen 183:184
Newline 184:185
Let 189:192
Ident 193:198 text="words"
Assign 199:200
LBrace 201:202
Str 202:205 T("x")
Colon 205:206
Str 207:211 T("v1")
Comma 211:212
Str 213:216 T("y")
Colon 216:217
Str 218:222 T("v2")
RBrace 222:223
Newline 223:224
For 228:231
LParen 232:233
LParen 233:234
Ident 234:237 text="key"
Comma 237:238
Ident 239:244 text="value"
RParen 244:245
In 246:248
Ident 249:254 text="words"
RParen 254:255
LBrace 256:257
Newline 257:258
Ident 266:273 text="println"
LParen 273:274
Str 274:289 E[Ident 276:279 text="key"; Eof 280:280] T("=") E[Ident 282:287 text="value"; Eof 288:288]
RParen 289:290
Newline 290:291
RBrace 295:296
Newline 296:297
RBrace 297:298
Eof 298:298

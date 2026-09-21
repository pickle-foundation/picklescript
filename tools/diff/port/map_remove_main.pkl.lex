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
RBrace 39:40
Newline 40:41
Let 45:48
Ident 49:56 text="removed"
Assign 57:58
Ident 59:60 text="m"
Dot 60:61
Ident 61:67 text="remove"
LParen 67:68
Str 68:71 T("a")
RParen 71:72
Newline 72:73
Let 77:80
Ident 81:85 text="hasa"
Assign 86:87
Ident 88:89 text="m"
Dot 89:90
Ident 90:93 text="has"
LParen 93:94
Str 94:97 T("a")
RParen 97:98
Newline 98:99
Let 103:106
Ident 107:111 text="hasb"
Assign 112:113
Ident 114:115 text="m"
Dot 115:116
Ident 116:119 text="has"
LParen 119:120
Str 120:123 T("b")
RParen 123:124
Newline 124:125
Let 129:132
Ident 133:138 text="count"
Assign 139:140
Ident 141:144 text="len"
LParen 144:145
Ident 145:146 text="m"
Dot 146:147
Ident 147:153 text="values"
LParen 153:154
RParen 154:155
RParen 155:156
Newline 156:157
If 161:163
LParen 164:165
Let 165:168
Ident 169:173 text="some"
LParen 173:174
Ident 174:175 text="v"
RParen 175:176
Assign 177:178
Ident 179:186 text="removed"
RParen 186:187
LBrace 188:189
Newline 189:190
Ident 198:205 text="println"
LParen 205:206
Str 206:259 T("removed ") E[Ident 216:217 text="v"; Eof 218:218] T(" has-a=") E[Ident 226:230 text="hasa"; Eof 231:231] T(" has-b=") E[Ident 239:243 text="hasb"; Eof 244:244] T(" count=") E[Ident 252:257 text="count"; Eof 258:258]
RParen 259:260
Newline 260:261
RBrace 265:266
Else 267:271
LBrace 272:273
Newline 273:274
Ident 282:289 text="println"
LParen 289:290
Str 290:307 T("unexpected none")
RParen 307:308
Newline 308:309
RBrace 313:314
Newline 314:315
Let 319:322
Ident 323:327 text="gone"
Assign 328:329
Ident 330:331 text="m"
Dot 331:332
Ident 332:338 text="remove"
LParen 338:339
Str 339:348 T("missing")
RParen 348:349
QuestionQuestion 350:352
Minus 353:354
Number 354:355 text="1" suf="" isf=0 int=1
Newline 355:356
Ident 360:367 text="println"
LParen 367:368
Str 368:381 T("gone ") E[Ident 375:379 text="gone"; Eof 380:380]
RParen 381:382
Newline 382:383
RBrace 383:384
Eof 384:384

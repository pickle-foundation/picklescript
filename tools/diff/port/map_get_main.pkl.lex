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
Ident 49:52 text="got"
Assign 53:54
Ident 55:56 text="m"
Dot 56:57
Get 57:60
LParen 60:61
Str 61:64 T("a")
RParen 64:65
Newline 65:66
If 70:72
LParen 73:74
Let 74:77
Ident 78:82 text="some"
LParen 82:83
Ident 83:84 text="v"
RParen 84:85
Assign 86:87
Ident 88:91 text="got"
RParen 91:92
LBrace 93:94
Newline 94:95
Let 103:106
Ident 107:114 text="missing"
Assign 115:116
Ident 117:118 text="m"
Dot 118:119
Get 119:122
LParen 122:123
Str 123:129 T("nope")
RParen 129:130
QuestionQuestion 131:133
Minus 134:135
Number 135:136 text="1" suf="" isf=0 int=1
Newline 136:137
Ident 145:152 text="println"
LParen 152:153
Str 153:199 T("got ") E[Ident 159:160 text="v"; Eof 161:161] T(" missing ") E[Ident 171:178 text="missing"; Eof 179:179] T(" has-b=") E[Ident 187:188 text="m"; Dot 188:189; Ident 189:192 text="has"; LParen 192:193; Str 193:196 T("b"); RParen 196:197; Eof 198:198]
RParen 199:200
Newline 200:201
RBrace 205:206
Else 207:211
LBrace 212:213
Newline 213:214
Ident 222:229 text="println"
LParen 229:230
Str 230:247 T("unexpected none")
RParen 247:248
Newline 248:249
RBrace 253:254
Newline 254:255
RBrace 255:256
Eof 256:256

Class 0:5
Ident 6:10 text="Hero"
LBrace 11:12
Newline 12:13
Static 17:23
Var 24:27
Ident 28:35 text="counter"
Colon 35:36
Ident 37:40 text="int"
Assign 41:42
Number 43:44 text="0" suf="" isf=0 int=0
Newline 44:45
Var 49:52
Ident 53:57 text="name"
Colon 57:58
Ident 59:65 text="string"
Newline 65:66
Var 70:73
Ident 74:79 text="level"
Colon 79:80
Ident 81:84 text="int"
Newline 84:85
RBrace 85:86
Newline 86:87
Newline 87:88
Fn 88:90
Ident 91:95 text="hero"
LParen 95:96
Ident 96:97 text="n"
Colon 97:98
Ident 99:105 text="string"
RParen 105:106
Arrow 107:109
Ident 110:114 text="Hero"
LBrace 115:116
Newline 116:117
Ident 121:125 text="Hero"
Dot 125:126
Ident 126:133 text="counter"
Assign 134:135
Ident 136:140 text="Hero"
Dot 140:141
Ident 141:148 text="counter"
Plus 149:150
Number 151:152 text="1" suf="" isf=0 int=1
Newline 152:153
Ident 157:161 text="Hero"
LParen 161:162
Ident 162:163 text="n"
Comma 163:164
Number 165:166 text="1" suf="" isf=0 int=1
RParen 166:167
Newline 167:168
RBrace 168:169
Newline 169:170
Newline 170:171
Class 171:176
Ident 177:183 text="Legacy"
LBrace 184:185
Newline 185:186
Static 190:196
Var 197:200
Ident 201:205 text="flag"
Colon 205:206
Ident 207:213 text="string"
Assign 214:215
Str 216:221 T("old")
Newline 221:222
Var 226:229
Ident 230:232 text="id"
Colon 232:233
Ident 234:237 text="int"
Newline 237:238
RBrace 238:239
Newline 239:240
Newline 240:241
Ident 241:249 text="describe"
LParen 249:250
Str 250:270 T("multi-file classes")
Comma 270:271
LBrace 272:273
Newline 273:274
Ident 278:282 text="test"
LParen 282:283
Str 283:310 T("hero statics are per-file")
Comma 310:311
LBrace 312:313
Newline 313:314
Ident 322:328 text="expect"
LParen 328:329
Ident 329:333 text="Hero"
Dot 333:334
Ident 334:341 text="counter"
RParen 341:342
Dot 342:343
Ident 343:347 text="toBe"
LParen 347:348
Number 348:349 text="0" suf="" isf=0 int=0
RParen 349:350
Newline 350:351
Ident 359:365 text="expect"
LParen 365:366
Ident 366:370 text="hero"
LParen 370:371
Str 371:378 T("alice")
RParen 378:379
Dot 379:380
Ident 380:384 text="name"
RParen 384:385
Dot 385:386
Ident 386:390 text="toBe"
LParen 390:391
Str 391:398 T("alice")
RParen 398:399
Newline 399:400
Ident 408:414 text="expect"
LParen 414:415
Ident 415:419 text="Hero"
Dot 419:420
Ident 420:427 text="counter"
RParen 427:428
Dot 428:429
Ident 429:433 text="toBe"
LParen 433:434
Number 434:435 text="1" suf="" isf=0 int=1
RParen 435:436
Newline 436:437
RBrace 441:442
RParen 442:443
Newline 443:444
Newline 444:445
Ident 449:453 text="test"
LParen 453:454
Str 454:476 T("second class in file")
Comma 476:477
LBrace 478:479
Newline 479:480
Ident 488:494 text="expect"
LParen 494:495
Ident 495:501 text="Legacy"
Dot 501:502
Ident 502:506 text="flag"
RParen 506:507
Dot 507:508
Ident 508:512 text="toBe"
LParen 512:513
Str 513:518 T("old")
RParen 518:519
Newline 519:520
Ident 528:534 text="expect"
LParen 534:535
Ident 535:541 text="Legacy"
LParen 541:542
Number 542:543 text="9" suf="" isf=0 int=9
RParen 543:544
Dot 544:545
Ident 545:547 text="id"
RParen 547:548
Dot 548:549
Ident 549:553 text="toBe"
LParen 553:554
Number 554:555 text="9" suf="" isf=0 int=9
RParen 555:556
Newline 556:557
RBrace 561:562
RParen 562:563
Newline 563:564
RBrace 564:565
RParen 565:566
Eof 566:566

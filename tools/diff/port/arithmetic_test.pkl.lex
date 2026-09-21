Fn 0:2
Ident 3:6 text="add"
LParen 6:7
Ident 7:8 text="a"
Colon 8:9
Ident 10:13 text="int"
Comma 13:14
Ident 15:16 text="b"
Colon 16:17
Ident 18:21 text="int"
RParen 21:22
Arrow 23:25
Ident 26:29 text="int"
LBrace 30:31
Newline 31:32
Ident 36:37 text="a"
Plus 38:39
Ident 40:41 text="b"
Newline 41:42
RBrace 42:43
Newline 43:44
Newline 44:45
Ident 45:49 text="test"
Fn 50:52
Ident 53:65 text="add_commutes"
LParen 65:66
RParen 66:67
LBrace 68:69
Newline 69:70
Ident 74:80 text="assert"
LParen 80:81
Ident 81:84 text="add"
LParen 84:85
Number 85:86 text="2" suf="" isf=0 int=2
Comma 86:87
Number 88:89 text="3" suf="" isf=0 int=3
RParen 89:90
EqEq 91:93
Ident 94:97 text="add"
LParen 97:98
Number 98:99 text="3" suf="" isf=0 int=3
Comma 99:100
Number 101:102 text="2" suf="" isf=0 int=2
RParen 102:103
RParen 103:104
Newline 104:105
RBrace 105:106
Newline 106:107
Newline 107:108
Ident 108:112 text="test"
Fn 113:115
Ident 116:131 text="add_known_value"
LParen 131:132
RParen 132:133
LBrace 134:135
Newline 135:136
Ident 140:146 text="assert"
LParen 146:147
Ident 147:150 text="add"
LParen 150:151
Number 151:152 text="4" suf="" isf=0 int=4
Comma 152:153
Number 154:155 text="5" suf="" isf=0 int=5
RParen 155:156
EqEq 157:159
Number 160:161 text="9" suf="" isf=0 int=9
Comma 161:162
Str 163:175 T("expected 9")
RParen 175:176
Newline 176:177
RBrace 177:178
Newline 178:179
Newline 179:180
Ident 180:184 text="test"
Fn 185:187
Ident 188:204 text="strings_are_utf8"
LParen 204:205
RParen 205:206
LBrace 207:208
Newline 208:209
Let 213:216
Ident 217:218 text="s"
Assign 219:220
Str 221:235 T("hello, world")
Newline 235:236
Ident 240:246 text="assert"
LParen 246:247
Ident 247:250 text="len"
LParen 250:251
Ident 251:252 text="s"
RParen 252:253
EqEq 254:256
Number 257:259 text="12" suf="" isf=0 int=12
Comma 259:260
Str 261:275 T("wrong length")
RParen 275:276
Newline 276:277
RBrace 277:278
Newline 278:279
Newline 279:280
Ident 280:284 text="test"
Fn 285:287
Ident 288:313 text="mixed_numeric_comparisons"
LParen 313:314
RParen 314:315
LBrace 316:317
Newline 317:318
Ident 322:328 text="assert"
LParen 328:329
Number 329:330 text="1" suf="" isf=0 int=1
Lt 331:332
Number 333:334 text="2" suf="" isf=0 int=2
RParen 334:335
Newline 335:336
Ident 340:346 text="assert"
LParen 346:347
Number 347:350 text="2.5" suf="" isf=1 int=-
Gt 351:352
Number 353:354 text="2" suf="" isf=0 int=2
RParen 354:355
Newline 355:356
Ident 360:366 text="assert"
LParen 366:367
Number 367:368 text="3" suf="" isf=0 int=3
Ge 369:371
Number 372:375 text="3.0" suf="" isf=1 int=-
RParen 375:376
Newline 376:377
Ident 381:387 text="assert"
LParen 387:388
Number 388:389 text="1" suf="" isf=0 int=1
NotEq 390:392
Number 393:396 text="1.5" suf="" isf=1 int=-
RParen 396:397
Newline 397:398
RBrace 398:399
Newline 399:400
Newline 400:401
Ident 401:405 text="test"
Fn 406:408
Ident 409:425 text="char_comparisons"
LParen 425:426
RParen 426:427
LBrace 428:429
Newline 429:430
Ident 434:440 text="assert"
LParen 440:441
Char 441:444 ch='a' int=97
Lt 445:446
Char 447:450 ch='z' int=122
RParen 450:451
Newline 451:452
Ident 456:462 text="assert"
LParen 462:463
Char 463:466 ch='z' int=122
Ge 467:469
Char 470:473 ch='a' int=97
RParen 473:474
Newline 474:475
Ident 479:485 text="assert"
LParen 485:486
Char 486:489 ch='m' int=109
NotEq 490:492
Char 493:496 ch='M' int=77
RParen 496:497
Newline 497:498
RBrace 498:499
Newline 499:500
Newline 500:501
Ident 501:505 text="test"
Fn 506:508
Ident 509:533 text="string_and_bool_equality"
LParen 533:534
RParen 534:535
LBrace 536:537
Newline 537:538
Let 542:545
Ident 546:547 text="s"
Assign 548:549
Str 550:555 T("abc")
Newline 555:556
Ident 560:566 text="assert"
LParen 566:567
Ident 567:568 text="s"
EqEq 569:571
Str 572:577 T("abc")
RParen 577:578
Newline 578:579
Ident 583:589 text="assert"
LParen 589:590
Ident 590:591 text="s"
NotEq 592:594
Str 595:600 T("abd")
RParen 600:601
Newline 601:602
Ident 606:612 text="assert"
LParen 612:613
True 613:617
NotEq 618:620
False 621:626
RParen 626:627
Newline 627:628
Ident 632:638 text="assert"
LParen 638:639
LParen 639:640
Number 640:641 text="1" suf="" isf=0 int=1
Lt 642:643
Number 644:645 text="2" suf="" isf=0 int=2
RParen 645:646
EqEq 647:649
True 650:654
RParen 654:655
Newline 655:656
RBrace 656:657
Eof 657:657

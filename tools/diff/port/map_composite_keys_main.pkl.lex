Class 0:5
Ident 6:8 text="Pt"
LBrace 9:10
Newline 10:11
Ident 15:16 text="x"
Colon 16:17
Ident 18:21 text="int"
Newline 21:22
Ident 26:27 text="y"
Colon 27:28
Ident 29:32 text="int"
Newline 32:33
RBrace 33:34
Newline 34:35
Newline 35:36
Enum 36:40
Ident 41:46 text="Shape"
LBrace 47:48
Newline 48:49
Ident 53:59 text="Circle"
Newline 59:60
Ident 64:70 text="Square"
Newline 70:71
Ident 75:79 text="Rect"
LParen 79:80
Ident 80:81 text="w"
Colon 81:82
Ident 83:86 text="int"
Comma 86:87
Ident 88:89 text="h"
Colon 89:90
Ident 91:94 text="int"
RParen 94:95
Newline 95:96
RBrace 96:97
Newline 97:98
Newline 98:99
Fn 99:101
Ident 102:106 text="main"
LParen 106:107
RParen 107:108
LBrace 109:110
Newline 110:111
Var 115:118
Ident 119:120 text="m"
Assign 121:122
LBrace 123:124
LBracket 124:125
Number 125:126 text="1" suf="" isf=0 int=1
Comma 126:127
Number 128:129 text="2" suf="" isf=0 int=2
RBracket 129:130
Colon 130:131
Str 132:138 T("pair")
Comma 138:139
LBracket 140:141
Number 141:142 text="3" suf="" isf=0 int=3
Comma 142:143
Number 144:145 text="4" suf="" isf=0 int=4
RBracket 145:146
Colon 146:147
Str 148:155 T("other")
RBrace 155:156
Newline 156:157
Ident 161:168 text="println"
LParen 168:169
Str 169:198 T("a=") E[Ident 173:174 text="m"; LBracket 174:175; LBracket 175:176; Number 176:177 text="1" suf="" isf=0 int=1; Comma 177:178; Number 179:180 text="2" suf="" isf=0 int=2; RBracket 180:181; RBracket 181:182; Eof 183:183] T(" b=") E[Ident 187:188 text="m"; LBracket 188:189; LBracket 189:190; Number 190:191 text="3" suf="" isf=0 int=3; Comma 191:192; Number 193:194 text="4" suf="" isf=0 int=4; RBracket 194:195; RBracket 195:196; Eof 197:197]
RParen 198:199
Newline 199:200
Let 204:207
Ident 208:209 text="p"
Assign 210:211
Ident 212:214 text="Pt"
LParen 214:215
Number 215:216 text="5" suf="" isf=0 int=5
Comma 216:217
Number 218:219 text="6" suf="" isf=0 int=6
RParen 219:220
Newline 220:221
Var 225:228
Ident 229:234 text="byobj"
Assign 235:236
LBrace 237:238
Ident 238:239 text="p"
Colon 239:240
Number 241:243 text="10" suf="" isf=0 int=10
RBrace 243:244
Newline 244:245
Ident 249:256 text="println"
LParen 256:257
Str 257:280 T("obj=") E[Ident 263:268 text="byobj"; LBracket 268:269; Ident 269:271 text="Pt"; LParen 271:272; Number 272:273 text="5" suf="" isf=0 int=5; Comma 273:274; Number 275:276 text="6" suf="" isf=0 int=6; RParen 276:277; RBracket 277:278; Eof 279:279]
RParen 280:281
Newline 281:282
Var 286:289
Ident 290:292 text="es"
Assign 293:294
LBrace 295:296
Ident 296:301 text="Shape"
Dot 301:302
Ident 302:308 text="Circle"
Colon 308:309
Str 310:313 T("c")
Comma 313:314
Ident 315:320 text="Shape"
Dot 320:321
Ident 321:325 text="Rect"
LParen 325:326
Number 326:327 text="2" suf="" isf=0 int=2
Comma 327:328
Number 329:330 text="3" suf="" isf=0 int=3
RParen 330:331
Colon 331:332
Str 333:336 T("r")
RBrace 336:337
Newline 337:338
Ident 342:349 text="println"
LParen 349:350
Str 350:397 T("c=") E[Ident 354:356 text="es"; LBracket 356:357; Ident 357:362 text="Shape"; Dot 362:363; Ident 363:369 text="Circle"; RBracket 369:370; Eof 371:371] T(" r=") E[Ident 375:377 text="es"; LBracket 377:378; Ident 378:383 text="Shape"; Dot 383:384; Ident 384:388 text="Rect"; LParen 388:389; Number 389:390 text="2" suf="" isf=0 int=2; Comma 390:391; Number 392:393 text="3" suf="" isf=0 int=3; RParen 393:394; RBracket 394:395; Eof 396:396]
RParen 397:398
Newline 398:399
Ident 403:404 text="m"
LBracket 404:405
LBracket 405:406
Number 406:407 text="5" suf="" isf=0 int=5
Comma 407:408
Number 409:410 text="6" suf="" isf=0 int=6
RBracket 410:411
RBracket 411:412
Assign 413:414
Str 415:422 T("added")
Newline 422:423
Var 427:430
Ident 431:434 text="tot"
Assign 435:436
Number 437:438 text="0" suf="" isf=0 int=0
Newline 438:439
For 443:446
LParen 447:448
LParen 448:449
Ident 449:450 text="k"
Comma 450:451
Ident 452:453 text="v"
RParen 453:454
In 455:457
Ident 458:459 text="m"
RParen 459:460
LBrace 461:462
Newline 462:463
Ident 471:474 text="tot"
PlusEq 475:477
Ident 478:479 text="k"
LBracket 479:480
Number 480:481 text="0" suf="" isf=0 int=0
RBracket 481:482
Newline 482:483
RBrace 487:488
Newline 488:489
Ident 493:500 text="println"
LParen 500:501
Str 501:512 T("tot=") E[Ident 507:510 text="tot"; Eof 511:511]
RParen 512:513
Newline 513:514
Ident 518:525 text="println"
LParen 525:526
Str 526:549 T("has56=") E[Ident 534:535 text="m"; Dot 535:536; Ident 536:539 text="has"; LParen 539:540; LBracket 540:541; Number 541:542 text="5" suf="" isf=0 int=5; Comma 542:543; Number 544:545 text="6" suf="" isf=0 int=6; RBracket 545:546; RParen 546:547; Eof 548:548]
RParen 549:550
Newline 550:551
Ident 555:562 text="println"
LParen 562:563
Str 563:570 T("miss=")
Plus 571:572
LParen 573:574
Ident 574:575 text="m"
Dot 575:576
Get 576:579
LParen 579:580
LBracket 580:581
Number 581:582 text="9" suf="" isf=0 int=9
Comma 582:583
Number 584:585 text="9" suf="" isf=0 int=9
RBracket 585:586
RParen 586:587
QuestionQuestion 588:590
Str 591:594 T("-")
RParen 594:595
RParen 595:596
Newline 596:597
RBrace 597:598
Eof 598:598

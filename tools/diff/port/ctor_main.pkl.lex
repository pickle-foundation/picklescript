Class 0:5
Ident 6:20 text="SynthBelowBase"
LBrace 21:22
Newline 22:23
Var 27:30
Ident 31:32 text="g"
Colon 32:33
Ident 34:37 text="int"
Assign 38:39
Number 40:43 text="100" suf="" isf=0 int=100
Newline 43:44
Newline 44:45
Constructor 49:60
LParen 60:61
Ident 61:62 text="g"
Colon 62:63
Ident 64:67 text="int"
RParen 67:68
LBrace 69:70
Newline 70:71
This 79:83
Dot 83:84
Ident 84:85 text="g"
Assign 86:87
This 88:92
Dot 92:93
Ident 93:94 text="g"
Plus 95:96
Ident 97:98 text="g"
Newline 98:99
RBrace 103:104
Newline 104:105
RBrace 105:106
Newline 106:107
Newline 107:108
Class 108:113
Ident 114:123 text="SynthLeaf"
Extends 124:131
Ident 132:146 text="SynthBelowBase"
LBrace 147:148
Newline 148:149
Var 153:156
Ident 157:158 text="l"
Colon 158:159
Ident 160:163 text="int"
Newline 163:164
RBrace 164:165
Newline 165:166
Newline 166:167
Class 167:172
Ident 173:181 text="AutoLeaf"
Extends 182:189
Ident 190:204 text="SynthBelowBase"
LBrace 205:206
Newline 206:207
Var 211:214
Ident 215:216 text="a"
Colon 216:217
Ident 218:221 text="int"
Assign 222:223
Number 224:225 text="5" suf="" isf=0 int=5
Newline 225:226
Var 230:233
Ident 234:235 text="b"
Colon 235:236
Ident 237:240 text="int"
Newline 240:241
RBrace 241:242
Newline 242:243
Newline 243:244
Class 244:249
Ident 250:257 text="DeepMid"
Extends 258:265
Ident 266:280 text="SynthBelowBase"
LBrace 281:282
Newline 282:283
Var 287:290
Ident 291:292 text="m"
Colon 292:293
Ident 294:297 text="int"
Assign 298:299
Number 300:301 text="1" suf="" isf=0 int=1
Newline 301:302
RBrace 302:303
Newline 303:304
Newline 304:305
Class 305:310
Ident 311:319 text="DeepLeaf"
Extends 320:327
Ident 328:335 text="DeepMid"
LBrace 336:337
Newline 337:338
Var 342:345
Ident 346:347 text="d"
Colon 347:348
Ident 349:352 text="int"
Newline 352:353
RBrace 353:354
Newline 354:355
Newline 355:356
Fn 356:358
Ident 359:363 text="main"
LParen 363:364
RParen 364:365
LBrace 366:367
Newline 367:368
Var 372:375
Ident 376:377 text="s"
Assign 378:379
Ident 380:389 text="SynthLeaf"
LParen 389:390
Number 390:391 text="5" suf="" isf=0 int=5
Comma 391:392
Number 393:394 text="7" suf="" isf=0 int=7
RParen 394:395
Newline 395:396
Ident 400:407 text="println"
LParen 407:408
Ident 408:409 text="s"
Dot 409:410
Ident 410:411 text="g"
Comma 411:412
Ident 413:414 text="s"
Dot 414:415
Ident 415:416 text="l"
RParen 416:417
Newline 417:418
Var 422:425
Ident 426:427 text="t"
Assign 428:429
Ident 430:438 text="AutoLeaf"
LParen 438:439
Number 439:440 text="2" suf="" isf=0 int=2
Comma 440:441
Number 442:443 text="9" suf="" isf=0 int=9
RParen 443:444
Newline 444:445
Ident 449:456 text="println"
LParen 456:457
Ident 457:458 text="t"
Dot 458:459
Ident 459:460 text="g"
Comma 460:461
Ident 462:463 text="t"
Dot 463:464
Ident 464:465 text="a"
Comma 465:466
Ident 467:468 text="t"
Dot 468:469
Ident 469:470 text="b"
RParen 470:471
Newline 471:472
Var 476:479
Ident 480:481 text="u"
Assign 482:483
Ident 484:492 text="DeepLeaf"
LParen 492:493
Number 493:494 text="3" suf="" isf=0 int=3
Comma 494:495
Number 496:497 text="4" suf="" isf=0 int=4
RParen 497:498
Newline 498:499
Ident 503:510 text="println"
LParen 510:511
Ident 511:512 text="u"
Dot 512:513
Ident 513:514 text="g"
Comma 514:515
Ident 516:517 text="u"
Dot 517:518
Ident 518:519 text="m"
Comma 519:520
Ident 521:522 text="u"
Dot 522:523
Ident 523:524 text="d"
RParen 524:525
Newline 525:526
Ident 530:537 text="println"
LParen 537:538
Ident 538:539 text="s"
Is 540:542
Ident 543:557 text="SynthBelowBase"
Comma 557:558
Ident 559:560 text="t"
Is 561:563
Ident 564:578 text="SynthBelowBase"
Comma 578:579
Ident 580:581 text="u"
Is 582:584
Ident 585:599 text="SynthBelowBase"
RParen 599:600
Newline 600:601
RBrace 601:602
Eof 602:602

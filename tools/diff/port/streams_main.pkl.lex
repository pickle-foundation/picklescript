Fn 0:2
Ident 3:7 text="main"
LParen 7:8
RParen 8:9
LBrace 10:11
Newline 11:12
Let 16:19
Ident 20:24 text="path"
Assign 25:26
Str 27:59 T("tests/pickle/__stream_main.tmp")
Newline 59:60
Ident 64:70 text="delete"
LParen 70:71
Ident 71:75 text="path"
RParen 75:76
Newline 76:77
Let 81:84
Ident 85:86 text="s"
Assign 87:88
Ident 89:106 text="stream_open_write"
LParen 106:107
Ident 107:111 text="path"
RParen 111:112
Newline 112:113
Var 117:120
Ident 121:127 text="opened"
Assign 128:129
False 130:135
Newline 135:136
If 140:142
LParen 143:144
Let 144:147
Ident 148:152 text="some"
LParen 152:153
Ident 153:154 text="w"
RParen 154:155
Assign 156:157
Ident 158:159 text="s"
RParen 159:160
LBrace 161:162
Newline 162:163
Ident 171:177 text="opened"
Assign 178:179
True 180:184
Newline 184:185
Let 193:196
Ident 197:199 text="n1"
Assign 200:201
Ident 202:203 text="w"
Dot 203:204
Ident 204:209 text="write"
LParen 209:210
Ident 210:215 text="bytes"
LParen 215:216
Str 216:224 T("stream")
RParen 224:225
RParen 225:226
Newline 226:227
Let 235:238
Ident 239:241 text="n2"
Assign 242:243
Ident 244:245 text="w"
Dot 245:246
Ident 246:251 text="write"
LParen 251:252
Ident 252:257 text="bytes"
LParen 257:258
Str 258:265 T(" main")
RParen 265:266
RParen 266:267
Newline 267:268
Ident 276:277 text="w"
Dot 277:278
Ident 278:283 text="close"
LParen 283:284
RParen 284:285
Newline 285:286
Ident 294:301 text="println"
LParen 301:302
Str 302:319 T("wrote=") E[Ident 310:312 text="n1"; Plus 313:314; Ident 315:317 text="n2"; Eof 318:318]
RParen 319:320
Newline 320:321
RBrace 325:326
Newline 326:327
Ident 331:338 text="println"
LParen 338:339
Str 339:356 T("opened=") E[Ident 348:354 text="opened"; Eof 355:355]
RParen 356:357
Newline 357:358
Let 362:365
Ident 366:367 text="r"
Assign 368:369
Ident 370:386 text="stream_open_read"
LParen 386:387
Ident 387:391 text="path"
RParen 391:392
Newline 392:393
If 397:399
LParen 400:401
Let 401:404
Ident 405:409 text="some"
LParen 409:410
Ident 410:412 text="rd"
RParen 412:413
Assign 414:415
Ident 416:417 text="r"
RParen 417:418
LBrace 419:420
Newline 420:421
Var 429:432
Ident 433:436 text="out"
Assign 437:438
Str 439:441
Newline 441:442
While 450:455
LParen 456:457
True 457:461
RParen 461:462
LBrace 463:464
Newline 464:465
Let 477:480
Ident 481:486 text="chunk"
Assign 487:488
Ident 489:491 text="rd"
Dot 491:492
Ident 492:496 text="read"
LParen 496:497
Number 497:498 text="3" suf="" isf=0 int=3
RParen 498:499
Newline 499:500
If 512:514
LParen 515:516
Let 516:519
Ident 520:524 text="some"
LParen 524:525
Ident 525:526 text="c"
RParen 526:527
Assign 528:529
Ident 530:535 text="chunk"
RParen 535:536
LBrace 537:538
Newline 538:539
Ident 555:558 text="out"
PlusEq 559:561
Ident 562:565 text="str"
LParen 565:566
Ident 566:567 text="c"
RParen 567:568
Newline 568:569
RBrace 581:582
Else 583:587
LBrace 588:589
Newline 589:590
Break 606:611
Newline 611:612
RBrace 624:625
Newline 625:626
RBrace 634:635
Newline 635:636
Ident 644:646 text="rd"
Dot 646:647
Ident 647:652 text="close"
LParen 652:653
RParen 653:654
Newline 654:655
Ident 663:670 text="println"
LParen 670:671
Str 671:683 T("data=") E[Ident 678:681 text="out"; Eof 682:682]
RParen 683:684
Newline 684:685
RBrace 689:690
Newline 690:691
Let 695:698
Ident 699:703 text="out2"
Assign 704:705
Ident 706:719 text="stdout_stream"
LParen 719:720
RParen 720:721
Newline 721:722
Ident 726:733 text="println"
LParen 733:734
Str 734:766 T("console_flushed=") E[Ident 752:756 text="out2"; Dot 756:757; Ident 757:762 text="flush"; LParen 762:763; RParen 763:764; Eof 765:765]
RParen 766:767
Newline 767:768
Ident 772:778 text="delete"
LParen 778:779
Ident 779:783 text="path"
RParen 783:784
Newline 784:785
RBrace 785:786
Eof 786:786

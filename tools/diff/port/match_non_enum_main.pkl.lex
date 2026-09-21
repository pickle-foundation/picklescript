Fn 0:2
Ident 3:7 text="band"
LParen 7:8
Ident 8:9 text="n"
Colon 9:10
Ident 11:14 text="int"
RParen 14:15
Arrow 16:18
Ident 19:25 text="string"
LBrace 26:27
Newline 27:28
Match 32:37
LParen 38:39
Ident 39:40 text="n"
RParen 40:41
LBrace 42:43
Newline 43:44
Case 52:56
Ident 57:58 text="x"
If 59:61
Ident 62:63 text="x"
Lt 64:65
Number 66:67 text="0" suf="" isf=0 int=0
Arrow 68:70
Str 71:76 T("neg")
Newline 76:77
Case 85:89
Number 90:91 text="0" suf="" isf=0 int=0
Arrow 92:94
Str 95:101 T("zero")
Newline 101:102
Case 110:114
Ident 115:116 text="x"
If 117:119
Ident 120:121 text="x"
Gt 122:123
Number 124:125 text="9" suf="" isf=0 int=9
Arrow 126:128
Str 129:134 T("big")
Newline 134:135
Case 143:147
Ident 148:149 text="_"
Arrow 150:152
Str 153:160 T("small")
Newline 160:161
RBrace 165:166
Newline 166:167
RBrace 167:168
Newline 168:169
Newline 169:170
Fn 170:172
Ident 173:180 text="name_of"
LParen 180:181
Ident 181:182 text="k"
Colon 182:183
Ident 184:190 text="string"
RParen 190:191
Arrow 192:194
Ident 195:201 text="string"
LBrace 202:203
Newline 203:204
Match 208:213
LParen 214:215
Ident 215:216 text="k"
RParen 216:217
LBrace 218:219
Newline 219:220
Case 228:232
Str 233:237 T("up")
Arrow 238:240
Str 241:248 T("north")
Newline 248:249
Case 257:261
Str 262:268 T("down")
Arrow 269:271
Str 272:279 T("south")
Newline 279:280
Case 288:292
Ident 293:294 text="_"
Arrow 295:297
Str 298:301 T("?")
Newline 301:302
RBrace 306:307
Newline 307:308
RBrace 308:309
Newline 309:310
Newline 310:311
Fn 311:313
Ident 314:320 text="unwrap"
LParen 320:321
Ident 321:322 text="m"
Colon 322:323
Ident 324:327 text="int"
Question 327:328
RParen 328:329
Arrow 330:332
Ident 333:336 text="int"
LBrace 337:338
Newline 338:339
Match 343:348
LParen 349:350
Ident 350:351 text="m"
RParen 351:352
LBrace 353:354
Newline 354:355
Case 363:367
Ident 368:372 text="some"
LParen 372:373
Ident 373:374 text="v"
RParen 374:375
Arrow 376:378
Ident 379:380 text="v"
Newline 380:381
Case 389:393
None 394:398
Arrow 399:401
Number 402:403 text="0" suf="" isf=0 int=0
Newline 403:404
RBrace 408:409
Newline 409:410
RBrace 410:411
Newline 411:412
Newline 412:413
Fn 413:415
Ident 416:420 text="pick"
LParen 420:421
Ident 421:422 text="m"
Colon 422:423
Ident 424:427 text="int"
Question 427:428
Comma 428:429
Ident 430:431 text="f"
Colon 431:432
Ident 433:436 text="int"
RParen 436:437
Arrow 438:440
Ident 441:444 text="int"
LBrace 445:446
Newline 446:447
If 451:453
LParen 454:455
Let 455:458
Ident 459:463 text="some"
LParen 463:464
Ident 464:465 text="v"
RParen 465:466
Assign 467:468
Ident 469:470 text="m"
RParen 470:471
LBrace 472:473
Newline 473:474
Ident 482:483 text="v"
Newline 483:484
RBrace 488:489
Else 490:494
LBrace 495:496
Newline 496:497
Ident 505:506 text="f"
Newline 506:507
RBrace 511:512
Newline 512:513
RBrace 513:514
Newline 514:515
Newline 515:516
Fn 516:518
Ident 519:523 text="main"
LParen 523:524
RParen 524:525
LBrace 526:527
Newline 527:528
Ident 532:539 text="println"
LParen 539:540
Ident 540:544 text="band"
LParen 544:545
Minus 545:546
Number 546:547 text="2" suf="" isf=0 int=2
RParen 547:548
Plus 549:550
Str 551:554 T("|")
Plus 555:556
Ident 557:561 text="band"
LParen 561:562
Number 562:563 text="0" suf="" isf=0 int=0
RParen 563:564
Plus 565:566
Str 567:570 T("|")
Plus 571:572
Ident 573:577 text="band"
LParen 577:578
Number 578:579 text="5" suf="" isf=0 int=5
RParen 579:580
Plus 581:582
Str 583:586 T("|")
Plus 587:588
Ident 589:593 text="band"
LParen 593:594
Number 594:596 text="42" suf="" isf=0 int=42
RParen 596:597
RParen 597:598
Newline 598:599
Ident 603:610 text="println"
LParen 610:611
Ident 611:618 text="name_of"
LParen 618:619
Str 619:623 T("up")
RParen 623:624
Plus 625:626
Str 627:630 T("|")
Plus 631:632
Ident 633:640 text="name_of"
LParen 640:641
Str 641:647 T("down")
RParen 647:648
Plus 649:650
Str 651:654 T("|")
Plus 655:656
Ident 657:664 text="name_of"
LParen 664:665
Str 665:671 T("left")
RParen 671:672
RParen 672:673
Newline 673:674
Ident 678:685 text="println"
LParen 685:686
Ident 686:692 text="unwrap"
LParen 692:693
None 693:697
RParen 697:698
RParen 698:699
Newline 699:700
Ident 704:711 text="println"
LParen 711:712
Ident 712:718 text="unwrap"
LParen 718:719
Number 719:720 text="7" suf="" isf=0 int=7
As 721:723
Question 723:724
Ident 725:728 text="int"
RParen 728:729
RParen 729:730
Newline 730:731
Ident 735:742 text="println"
LParen 742:743
Ident 743:749 text="unwrap"
LParen 749:750
Number 750:751 text="3" suf="" isf=0 int=3
As 752:754
Question 754:755
Ident 756:759 text="int"
RParen 759:760
RParen 760:761
Newline 761:762
Ident 766:773 text="println"
LParen 773:774
Ident 774:778 text="pick"
LParen 778:779
None 779:783
Comma 783:784
Minus 785:786
Number 786:787 text="1" suf="" isf=0 int=1
RParen 787:788
Comma 788:789
Ident 790:794 text="pick"
LParen 794:795
Number 795:796 text="9" suf="" isf=0 int=9
As 797:799
Question 799:800
Ident 801:804 text="int"
Comma 804:805
Minus 806:807
Number 807:808 text="1" suf="" isf=0 int=1
RParen 808:809
RParen 809:810
Newline 810:811
RBrace 811:812
Eof 812:812

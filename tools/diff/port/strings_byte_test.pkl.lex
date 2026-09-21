Ident 0:4 text="test"
Fn 5:7
Ident 8:24 text="len_counts_bytes"
LParen 24:25
RParen 25:26
LBrace 27:28
Newline 28:29
Ident 33:39 text="expect"
LParen 39:40
Ident 40:43 text="len"
LParen 43:44
Str 44:48 T("é")
RParen 48:49
RParen 49:50
Dot 50:51
Ident 51:55 text="toBe"
LParen 55:56
Number 56:57 text="2" suf="" isf=0 int=2
RParen 57:58
Newline 58:59
Ident 63:69 text="expect"
LParen 69:70
Ident 70:73 text="len"
LParen 73:74
Str 74:79 T("abc")
RParen 79:80
RParen 80:81
Dot 81:82
Ident 82:86 text="toBe"
LParen 86:87
Number 87:88 text="3" suf="" isf=0 int=3
RParen 88:89
Newline 89:90
RBrace 90:91
Newline 91:92
Newline 92:93
Ident 93:97 text="test"
Fn 98:100
Ident 101:124 text="index_yields_utf8_bytes"
LParen 124:125
RParen 125:126
LBrace 127:128
Newline 128:129
Ident 133:139 text="expect"
LParen 139:140
Str 140:144 T("é")
LBracket 144:145
Number 145:146 text="0" suf="" isf=0 int=0
RBracket 146:147
EqEq 148:150
Number 151:154 text="195" suf="" isf=0 int=195
RParen 154:155
Dot 155:156
Ident 156:160 text="toBe"
LParen 160:161
True 161:165
RParen 165:166
Newline 166:167
Ident 171:177 text="expect"
LParen 177:178
Str 178:182 T("é")
LBracket 182:183
Number 183:184 text="1" suf="" isf=0 int=1
RBracket 184:185
EqEq 186:188
Number 189:192 text="169" suf="" isf=0 int=169
RParen 192:193
Dot 193:194
Ident 194:198 text="toBe"
LParen 198:199
True 199:203
RParen 203:204
Newline 204:205
Ident 209:215 text="expect"
LParen 215:216
Str 216:221 T("abc")
LBracket 221:222
Number 222:223 text="1" suf="" isf=0 int=1
RBracket 223:224
EqEq 225:227
Number 228:230 text="98" suf="" isf=0 int=98
RParen 230:231
Dot 231:232
Ident 232:236 text="toBe"
LParen 236:237
True 237:241
RParen 241:242
Newline 242:243
Ident 247:253 text="expect"
LParen 253:254
Str 254:259 T("abc")
LBracket 259:260
Number 260:261 text="1" suf="" isf=0 int=1
RBracket 261:262
EqEq 263:265
Char 266:269 ch='b' int=98
RParen 269:270
Dot 270:271
Ident 271:275 text="toBe"
LParen 275:276
True 276:280
RParen 280:281
Newline 281:282
RBrace 282:283
Newline 283:284
Newline 284:285
Ident 285:289 text="test"
Fn 290:292
Ident 293:330 text="ascii_char_literal_compares_with_byte"
LParen 330:331
RParen 331:332
LBrace 333:334
Newline 334:335
Let 339:342
Ident 343:344 text="s"
Assign 345:346
Str 347:351 T("ab")
Newline 351:352
Ident 356:362 text="expect"
LParen 362:363
Ident 363:364 text="s"
LBracket 364:365
Number 365:366 text="0" suf="" isf=0 int=0
RBracket 366:367
EqEq 368:370
Char 371:374 ch='a' int=97
RParen 374:375
Dot 375:376
Ident 376:380 text="toBe"
LParen 380:381
True 381:385
RParen 385:386
Newline 386:387
Ident 391:397 text="expect"
LParen 397:398
Char 398:401 ch='a' int=97
EqEq 402:404
Ident 405:406 text="s"
LBracket 406:407
Number 407:408 text="0" suf="" isf=0 int=0
RBracket 408:409
RParen 409:410
Dot 410:411
Ident 411:415 text="toBe"
LParen 415:416
True 416:420
RParen 420:421
Newline 421:422
RBrace 422:423
Newline 423:424
Newline 424:425
Ident 425:429 text="test"
Fn 430:432
Ident 433:458 text="copy_loop_roundtrips_utf8"
LParen 458:459
RParen 459:460
LBrace 461:462
Newline 462:463
Var 467:470
Ident 471:474 text="out"
Assign 475:476
Str 477:479
Newline 479:480
For 484:487
LParen 488:489
Ident 489:490 text="c"
In 491:493
Str 494:498 T("é")
RParen 498:499
LBrace 500:501
Newline 501:502
Ident 510:513 text="out"
PlusEq 514:516
Str 517:522 E[Ident 519:520 text="c"; Eof 521:521]
Newline 522:523
RBrace 527:528
Newline 528:529
Ident 533:539 text="expect"
LParen 539:540
Ident 540:543 text="out"
RParen 543:544
Dot 544:545
Ident 545:549 text="toBe"
LParen 549:550
Str 550:554 T("é")
RParen 554:555
Newline 555:556
RBrace 556:557
Newline 557:558
Newline 558:559
Ident 559:563 text="test"
Fn 564:566
Ident 567:582 text="copy_loop_ascii"
LParen 582:583
RParen 583:584
LBrace 585:586
Newline 586:587
Var 591:594
Ident 595:598 text="out"
Assign 599:600
Str 601:603
Newline 603:604
For 608:611
LParen 612:613
Ident 613:614 text="c"
In 615:617
Str 618:623 T("abc")
RParen 623:624
LBrace 625:626
Newline 626:627
Ident 635:638 text="out"
PlusEq 639:641
Str 642:647 E[Ident 644:645 text="c"; Eof 646:646]
Newline 647:648
RBrace 652:653
Newline 653:654
Ident 658:664 text="expect"
LParen 664:665
Ident 665:668 text="out"
RParen 668:669
Dot 669:670
Ident 670:674 text="toBe"
LParen 674:675
Str 675:680 T("abc")
RParen 680:681
Newline 681:682
RBrace 682:683
Newline 683:684
Newline 684:685
Ident 685:689 text="test"
Fn 690:692
Ident 693:714 text="byte_compare_with_int"
LParen 714:715
RParen 715:716
LBrace 717:718
Newline 718:719
Let 723:726
Ident 727:728 text="s"
Assign 729:730
Str 731:738 T("hello")
Newline 738:739
Var 743:746
Ident 747:748 text="n"
Assign 749:750
Number 751:752 text="0" suf="" isf=0 int=0
Newline 752:753
For 757:760
LParen 761:762
Ident 762:763 text="b"
In 764:766
Ident 767:768 text="s"
RParen 768:769
LBrace 770:771
Newline 771:772
If 780:782
LParen 783:784
Ident 784:785 text="b"
EqEq 786:788
Number 789:792 text="104" suf="" isf=0 int=104
RParen 792:793
LBrace 794:795
Newline 795:796
Ident 808:809 text="n"
PlusEq 810:812
Number 813:814 text="1" suf="" isf=0 int=1
Newline 814:815
RBrace 823:824
Newline 824:825
RBrace 829:830
Newline 830:831
Ident 835:841 text="expect"
LParen 841:842
Ident 842:843 text="n"
RParen 843:844
Dot 844:845
Ident 845:849 text="toBe"
LParen 849:850
Number 850:851 text="1" suf="" isf=0 int=1
RParen 851:852
Newline 852:853
RBrace 853:854
Newline 854:855
Newline 855:856
Ident 856:860 text="test"
Fn 861:863
Ident 864:883 text="char_interp_is_utf8"
LParen 883:884
RParen 884:885
LBrace 886:887
Newline 887:888
Let 892:895
Ident 896:897 text="s"
Assign 898:899
Str 900:908 E[Char 902:906 ch='é' int=233; Eof 907:907]
Newline 908:909
Ident 913:919 text="expect"
LParen 919:920
Ident 920:923 text="len"
LParen 923:924
Ident 924:925 text="s"
RParen 925:926
RParen 926:927
Dot 927:928
Ident 928:932 text="toBe"
LParen 932:933
Number 933:934 text="2" suf="" isf=0 int=2
RParen 934:935
Newline 935:936
Ident 940:946 text="expect"
LParen 946:947
Ident 947:948 text="s"
RParen 948:949
Dot 949:950
Ident 950:954 text="toBe"
LParen 954:955
Str 955:959 T("é")
RParen 959:960
Newline 960:961
RBrace 961:962
Newline 962:963
Newline 963:964
Ident 964:968 text="test"
Fn 969:971
Ident 972:1006 text="byte_loop_counts_bytes_not_scalars"
LParen 1006:1007
RParen 1007:1008
LBrace 1009:1010
Newline 1010:1011
Var 1015:1018
Ident 1019:1020 text="n"
Assign 1021:1022
Number 1023:1024 text="0" suf="" isf=0 int=0
Newline 1024:1025
For 1029:1032
LParen 1033:1034
Ident 1034:1035 text="c"
In 1036:1038
Str 1039:1044 T("aé")
RParen 1044:1045
LBrace 1046:1047
Newline 1047:1048
Ident 1056:1057 text="n"
PlusEq 1058:1060
Number 1061:1062 text="1" suf="" isf=0 int=1
Newline 1062:1063
RBrace 1067:1068
Newline 1068:1069
Ident 1073:1079 text="expect"
LParen 1079:1080
Ident 1080:1081 text="n"
RParen 1081:1082
Dot 1082:1083
Ident 1083:1087 text="toBe"
LParen 1087:1088
Number 1088:1089 text="3" suf="" isf=0 int=3
RParen 1089:1090
Newline 1090:1091
RBrace 1091:1092
Eof 1092:1092

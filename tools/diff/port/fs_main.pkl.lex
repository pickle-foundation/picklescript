Fn 0:2
Ident 3:7 text="main"
LParen 7:8
RParen 8:9
LBrace 10:11
Newline 11:12
Let 16:19
Ident 20:24 text="path"
Assign 25:26
Str 27:55 T("tests/pickle/__fs_main.tmp")
Newline 55:56
Ident 60:70 text="write_file"
LParen 70:71
Ident 71:75 text="path"
Comma 75:76
Str 77:93 T("main: hello fs")
RParen 93:94
Newline 94:95
Let 99:102
Ident 103:107 text="data"
Assign 108:109
Ident 110:119 text="read_file"
LParen 119:120
Ident 120:124 text="path"
RParen 124:125
Newline 125:126
If 130:132
LParen 133:134
Let 134:137
Ident 138:142 text="some"
LParen 142:143
Ident 143:146 text="txt"
RParen 146:147
Assign 148:149
Ident 150:154 text="data"
RParen 154:155
LBrace 156:157
Newline 157:158
Ident 166:171 text="print"
LParen 171:172
Ident 172:175 text="txt"
RParen 175:176
Newline 176:177
Ident 185:192 text="println"
LParen 192:193
Str 193:195
RParen 195:196
Newline 196:197
RBrace 201:202
Else 203:207
LBrace 208:209
Newline 209:210
Ident 218:225 text="println"
LParen 225:226
Str 226:245 T("main: read failed")
RParen 245:246
Newline 246:247
RBrace 251:252
Newline 252:253
Ident 257:262 text="print"
LParen 262:263
Ident 263:274 text="file_exists"
LParen 274:275
Str 275:301 T("tests/pickle/fs_test.pkl")
RParen 301:302
RParen 302:303
Newline 303:304
Ident 308:315 text="println"
LParen 315:316
Str 316:318
RParen 318:319
Newline 319:320
Ident 324:329 text="print"
LParen 329:330
Ident 330:341 text="file_exists"
LParen 341:342
Str 342:378 T("tests/pickle/__fs_main_missing.tmp")
RParen 378:379
RParen 379:380
Newline 380:381
Ident 385:392 text="println"
LParen 392:393
Str 393:395
RParen 395:396
Newline 396:397
Ident 401:406 text="print"
LParen 406:407
Ident 407:416 text="read_file"
LParen 416:417
Str 417:453 T("tests/pickle/__fs_main_missing.tmp")
RParen 453:454
QuestionQuestion 455:457
Str 458:473 T("fallback:none")
RParen 473:474
Newline 474:475
Ident 479:486 text="println"
LParen 486:487
Str 487:489
RParen 489:490
Newline 490:491
Let 495:498
Ident 499:502 text="dir"
Assign 503:504
Str 505:533 T("tests/pickle/__fs_main_dir")
Newline 533:534
Ident 538:543 text="print"
LParen 543:544
Ident 544:549 text="mkdir"
LParen 549:550
Ident 550:553 text="dir"
RParen 553:554
RParen 554:555
Newline 555:556
Ident 560:567 text="println"
LParen 567:568
Str 568:570
RParen 570:571
Newline 571:572
Ident 576:581 text="print"
LParen 581:582
Ident 582:588 text="delete"
LParen 588:589
Str 589:605 E[Ident 591:594 text="dir"; Eof 595:595] T("/nope.txt")
RParen 605:606
RParen 606:607
Newline 607:608
Ident 612:619 text="println"
LParen 619:620
Str 620:622
RParen 622:623
Newline 623:624
Ident 628:638 text="write_file"
LParen 638:639
Str 639:652 E[Ident 641:644 text="dir"; Eof 645:645] T("/x.txt")
Comma 652:653
Str 654:657 T("x")
RParen 657:658
Newline 658:659
Let 663:666
Ident 667:669 text="es"
Assign 670:671
Ident 672:680 text="list_dir"
LParen 680:681
Ident 681:684 text="dir"
RParen 684:685
Newline 685:686
Var 690:693
Ident 694:695 text="n"
Assign 696:697
Number 698:699 text="0" suf="" isf=0 int=0
Newline 699:700
If 704:706
LParen 707:708
Let 708:711
Ident 712:716 text="some"
LParen 716:717
Ident 717:718 text="l"
RParen 718:719
Assign 720:721
Ident 722:724 text="es"
RParen 724:725
LBrace 726:727
Newline 727:728
Ident 736:737 text="n"
Assign 738:739
Ident 740:743 text="len"
LParen 743:744
Ident 744:745 text="l"
RParen 745:746
Newline 746:747
RBrace 751:752
Newline 752:753
Ident 757:762 text="print"
LParen 762:763
Ident 763:764 text="n"
RParen 764:765
Newline 765:766
Ident 770:777 text="println"
LParen 777:778
Str 778:780
RParen 780:781
Newline 781:782
Ident 786:791 text="print"
LParen 791:792
Ident 792:798 text="delete"
LParen 798:799
Str 799:812 E[Ident 801:804 text="dir"; Eof 805:805] T("/x.txt")
RParen 812:813
RParen 813:814
Newline 814:815
Ident 819:826 text="println"
LParen 826:827
Str 827:829
RParen 829:830
Newline 830:831
Ident 835:840 text="print"
LParen 840:841
Ident 841:847 text="delete"
LParen 847:848
Ident 848:851 text="dir"
RParen 851:852
RParen 852:853
Newline 853:854
Ident 858:865 text="println"
LParen 865:866
Str 866:868
RParen 868:869
Newline 869:870
Ident 874:879 text="print"
LParen 879:880
Ident 880:886 text="delete"
LParen 886:887
Ident 887:891 text="path"
RParen 891:892
RParen 892:893
Newline 893:894
Ident 898:905 text="println"
LParen 905:906
Str 906:908
RParen 908:909
Newline 909:910
Ident 914:919 text="print"
LParen 919:920
Ident 920:930 text="write_file"
LParen 930:931
Ident 931:935 text="path"
Comma 935:936
Str 937:950 T("regenerated")
RParen 950:951
RParen 951:952
Newline 952:953
Ident 957:964 text="println"
LParen 964:965
Str 965:967
RParen 967:968
Newline 968:969
RBrace 969:970
Eof 970:970

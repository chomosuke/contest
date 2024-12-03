#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    unused_labels,
    clippy::ptr_arg,
    clippy::comparison_chain,
    clippy::collapsible_else_if
)]
use core::hash::Hash;
use io::*;
use regex::Regex;
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader, Stdin, Stdout},
    iter,
    mem::{self, swap},
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, Range, RangeBounds,
    },
};

type I = i128;
type U = u128;

static input: &str = r###"},;who()^>',mul(594,203)~  ~*$'*select()mul(693,99)*>&()+{%{mul(225,584)when()why()#]}&mul(287,918)<from(332,448)<^:mul(296,804)'@why()'when(),do()(+%:(who(309,257)mul(402,955):-')]</how()'{mul(462,541)who()^'{]-mul(677,297):*)-)]mul(997,185)mul(159,913);where()'+>}^mul(368,284)!>mul(943,865){who() /mul(482,561)don't()<,}>what()why();why()mul(407,849)@-mul(516,359))%:*<~&,}from()do()how()^+#^]when()%why()mul(604,810)when()from()mul(688,243)< ?[-]-who()mul(321,988)<:(%~!>[how()mul(477,536)how()!;~'mul(151,994)};mul(980,874))mul(439,349)what()]%(!%+%mul(915,912)where()~~?[mul(260,395)-mul(232,933)?why()<-&,#do()^select()'why();where()why()when()<+mul(681,169)>{#:$who()mul(667,451)mul(808,838)mul(737,381) where()how(847,856)-$):why()mul(337,590)%*)from()/what()%)]who()don't()'+[}mul(169,314)#from()/mul(936,343)why()&^mul(635,808)what()#who()$!~mul(721,645)what()!$&mul(508,537)/'>;[select()mul(570,587)when()where()/why();@/}]>don't()how(),why()mul(832,893)<mul(890,548)+'mul(887,354)&$why()select()where()$[]'do()[mul(526,27)/:^~how()mul(184,719)~?%,mul(706,502)where()from(668,399)^$mul(952,705) (,}when()<mul(525,685)}/+-what()&+^mul(461,816)} mul(15,833)!){(why():}mul(452,37){,~mul(661,869)$what()]$ [do()[@[> >]mul(862,653),>/@)[:what()?mul(579,120)#<when()mul(611,837) from()!/mul(697,303)>mul(972,506)>!$why()'-#+mul(837,98)from(156,299)why()why()select()*where() from()mul(244,480)/when()mul(89,666)-mul(18,187))^?)/mul(581,521)what(632,250)}]where()what()$mul(255,85)%mul(63,827)@*&select(),+from()mul(588,403)#[# how():^from()mul(87,601)mul(894,913))+mul(999,830)^how()&who()when()mul(63,360)~:)'mul(886,820)#+when(637,501)when(),mul(826,841)<mul(871,637),[*#-mul(876,861): @^@mul(687,724)[^-?don't()+>when()-who()mul(590,783)mul(964,976){>what()>mul(13,712):@]']from()where()who()::mulhow()+'~{;who()mul(79,750)@mul(59,645-mul(117,5)!*how()+mul(934,599)&mul(259,433)!$who()who(173,704)mul(622,219)mul(799,40))%-&/]-mul(964,29)/'who() how(),mul(640,513);where()why()-((?select()what()mul(337,730)from()when()why(){from()?mul(89,919)<~'(~@-where()mul(307,659)?select()what()select(233,108)('~%mul(257,371)what()who()}[]?where()who()mul(678,224);'-~@why()$*~from()mul(93,137))why(526,42)!'{select()what()what()when()+mul(487,383)':mul(101,242)/how():]%why()what()[:-do()-*#]^,]'@#mul(274,908)*:where(385,486)++how(658,16) ('mulwho()~#)!,what()%how()when(),mul(82,590)~when()where()mul(675,232)from()~how()~~$@*!mul(374,877)?,,]:<why()[#mul(238,568)+#select()when()(what()[mul(288,92)+&mul(935,678)(-+mul(646,366)^when()from()#&where()>;]mul(525{what()]when()what()<@'#~mul(218,321)why()[mul(7,149))]when()'mul(602,958)from()~]/>, mul(185,660)$select(),('}@mul(858,378)who()]>+mul(994,816)![^when()/why(311,783)>+mul(477,340)[what(),[who()%mul(295,109)/+~who()from()% mul(470,357))}@',*%how()how(132,604)}mul(592,563)who(464,432)+>where();who()[)where()mul(21,898)where()how()where()) how()how()-mul(853,634)%what()who()?mul(104,46),when()mul(425,445)how()select()where()mul(812,255,-'mul(326,218)+from()%^mul(326,896)-@mul(459,929)'mul-&where()where()[%}where()when()-mul(385,422);)^what()#mul(452,336)<:why()^ mul(486,292)$-select()select()mul(271,185)mul(839,107)mul(428,910)?;select()/-~)$mul(227,340)who()[[:select()how()^{where()%mul(589,539)
^]%mul(381,668)don't();<what()/>[*)how()?mul(670,136) select()select(){>>]+mul(838,443)why()< [how()where()why()mul(122,722)where()!#?mul(500,988)~:>#mul(231,76);when()@?<what()+mul(375,131)%mul(142,344)+?;/do()!~who(){mul(798,493)^]]{mul(760,519)#;]who() {mul(731,849)$(::-don't();{[+%mul(92,201)when(463,392)mul(555,452)select()how()where(236,738)?-^-'where()mul(830,999)mul(275,204):&:')$mul(649,43)when()'];select()&don't()}!mul(61,760)+[&where()-! !)'mul(986,940) where()why()::^;+)mul(170,843)<[mul(80,442):){mul(844,466)why(599,707)*^@-@where()*don't();how();##how()->*mul(263,91){mul(681,948)[where()mul(405,666)<,mul(593,738)?:++]&from()select()~mul(124,503)what()]@-$!mul(628,744)+&@who(),>@[]mul(522,713)]#select()?why()when())$,~mul(262,399){from()/^ #mul(893,939)$^:/what()mul(190,162),@where()(@&why(768,548)~mul(486,353)>*(!why(762,229)^who()-*#mul(332,970)(;+^#select()/when()mul(340,741)mul(798,465)what()]when(415,81){@mul(878,392)-what()why(301,507))? (mul(368,530),( @!-{<mul(862,207)&}%from()mul(959,624)what()??select()mul(727,721)<mul(597,447)mul(380,969);mul(718,644)>why()mul(648]who() ^where()+>&why(){+mul(629,258) ;how()when()where()when(543,323)#>)!mul(874,15)what()when()>{;]where()mul(698,403))?!@*?where(288,133)why():mul(710,643)'^why()why()*){mul(53,898)@!(*/??{@-mul(929,921)mul(346,179));@],^[@mul(135,728)select()$@*']mul(617,892)%^who()>;*-%@mul(32,615)from()>-don't()% ~what()where()mul(857,690)[@&what()[%<&from()+mul(264,50)(''';mul(603,221)$&*,{+mul(779,669)mul(111,444)!)*(select()$@mul(340,10)(!~don't())-$where()who()~/from()$#mul(363,956),$^%mul(425,73); @)@{)@mul(927,538)<do()mul(263,610)%!$when()mul(795,738)when()&+select()[mul(193,857)what()^ ~^what()where()from(440,495)mul(180,738)don't()why(){mul(72,22);[^!&']}mul(796,290)']mul(117,806)mul(584~who()/]{%?$$['mul(902,609)@-#^'^where()-mul(482,194)mul(476what()!from()?+what(){$ ;<mul(685,45)what()?,who()'!$,why()mul(241,772);-select(167,72)select()when()?mul(447,333)when()*({:$>{ how()mul(541,949+]><)([mul(627,334)?#select()^:mul(838,568)[?where()'+>/don't()mul(225,275)how()]who()mul(674,310)mul(388,186)from()select(415,201)':#('<select()-mul(628,432)what()+:don't()mul(621,550)&,mul(936,206)!when(823,958)mul(421,850)what()how() do()$]mul(479,729){~+'!when(){who()don't()[&}+?%~</mul(135,326)%~)({mul(664,774),<^;+;}select()mul(94,609)&select()when()mul(953,750)</!+^*(mul(192,73) when();don't()*}//why()>when()who()~mul~@:;mul(55,655)how()&)+'%<>mul(862,203)what()<who(),why()?/?~mul(948,366)%what())how()? mul(248,555)>>;)&mul(196,780)>*mul(8,530)<(<#?%>!mul(963*$!+when()where(),what()~what(402,657)why()mul(282,587)mul(760who()&+-&^ #mul(163,582))where()#;(?%do(){}who()what())mul(437,16)*(when()^mul(58,667)!from();:& -]@<mul(837,19)*why(689,440),*/?)mul(202,275)do():,<$from()(from()(mul(461,612)where()what()[)]%#mul(172,865)@select()*[<{#@mul(109,57))mul(868,620)$)+!mul(403,491):?(:&)from()(mul(678,566)'mul(591,957)>^mul(853,288)()'*++%<>mul(914,566)$>when()mul(15,420)&(mul(223,567why()?where()mul(448,495)mul(139,496)]/*,<mul(10,674)what())!,,do()]>!};mul(177,399)%mul(39,651)]:$[}?do()!>why()mul(417,729)}$-from()mul(520,450)mul(265,783){^/mul(772,557){{who();from()mul(493,714)[(mul(35,779!&?(&mul(569,865)
}where()'$:^,mul(786,269),% )^mul(879,113'how()@(/$%@/^mul(116,775)>'mul(982,672)$]when()]*mul(837]from()($@#@;how()why()mul(886,777)what()%who()when()who()&mul(21,661):&from()^select()--who()mul(853,286)mul(400,239)(^{}mul(831,506)mul(863,343)%don't()+]!@where(),from()mul(231,737)}:select()^!]}when(52,692)*~mul(370,465)-([select()%&select()mul(469,210){@,&{$mul(175,116)mul(609,363)':),@>[(,,mul(399,62)who();;what(470,862)when()mul(976,776)from()mul(7,424)$+(mul(196,54))>'why(),why()(''mul(401,800),@-what()!;}mul(571,161)<{~how()from()mul(578,181)mul(656,101)?%,where()why()mul(994,308)^select()@'&mul(326,809)}select()%what()%who()mul(905,918)(-?@mul(607,194))</~[from()select()/{mul(562,631)[mul(15,267)mul(501,382)(^(-!mul(669,954)--why()-mul(53,4)({when(){,/?don't())from():{~/,{/;mul(960,812)select()don't()?()]-mulwhy()/]:?mul(343,50)[mul(458,913)mul(761,672)]%when()don't()%>$who()when()[mul(400,976)%when()$[~mul(177,579)when()?what())why()what()*]select()mul(151,60)++mul(128,963)mul(139,337){mul(590,594)mul(103,702)>*&]##?mul(871,345)when()-/when(942,63)why()/mul(633,445)mul(847,599)??+:@&when()mul(646,120)[<from(930,22)where()?select()]where()when(34,121)/mul(795,773when()>>][select()mul(743,178)(~mul(684,203)who()?}why()*mul(229,310)*~$how()@when()-mul(701where()'when()how()(from()}>mul(641,487)$+:]mul(956,874)why()mul(353,956)mul(983,116what()+@who()don't()!where()from())),('mul(193,660)how()what()>%(( (<mul(534,159)$/when())^;mul(211,383)>[:[(?mul(682,785)]!@from()mul?{(>:^when()mul(544,177)$when(),/from()/why(216,114)(how()mul(548,300)who()$[from()where()/mul(842,217)why()<#when()/)< mul(20,214)?-')[how()*how()select()mul(762,33)~}mul(104,195) $)from()mul(752,787)select()~who()how()/mul(592,312)[{mul(19,834)from()when()&mul(429,262)&[[%mul(403,815);?,<$mul(585,835)%select()/'mul(487,715)]{>~,+who()from()what(230,638);mul(120,525)how()mul(357,111)who()%mul(348,302)mul(49,57)when()/:)}+;{mul(454,979)where()mul(592,663)mul(629,69){[)}/select()mul(937,214)~select(175,966)how()[~when():$mul(256,835)select()why(366,997)/({mul(968,356)),mul(911,215)&&:}why()*mul(817,467) $'] )-mul(539,632)[>mul(150,228)(& when()from()'mul(513,805){@*mul(386,351)-'?)where()(/$from()mulwhen(){mul(565,559)!$;} mul(584,177)'* who(779,974)-,where()mul(696,100)!!mul(905,665)$$what():'?$'}mul(244,781)'from(562,990)?what()<where()]select();mul^why(922,140)]+{?!<when()<^mul(221,231)who()>~mul(997,348)}(select()$-;what()^mul(208,506)who()*;<when(508,605)who()^select()when()-mul]'mul(702,347)from()don't()where()+(who()+/~mul(938,262)>)->:mul(175,377)>?%-})?$-what()mul(274,842)select()&!,+ what()$:select()mul(624,649)do()*~mul(24'{mul(607,613))mul(847,392)why()'[]+who()*from(35,544)do()'#@[!,what()mul(936,485)@]}why(487,34)<'why()why()!mul(507,185)(*>@}from()when()how()mul(126,880),#:from()mul(37,739)+;select()#mul(33,245)<where())])&~<$mul(433,395)/''-/*how()'?,mul(279select(410,45)+what(328,292)from()&do()mul(330,600)#{who()(--mul(821,674)[where();(don't()how()@/mul(138,856)!]:(~mul(178,997)/mul(739,585)?mul(344,362)]from()mul(446,222)^:from()~where()?<['?mul(930,705)from()&mul(274,695)~,<where()%};*mul(956,469)[-?mul(75,189)*-:mul(966,410))[where()]!mul(774,528)when(424,975)mul(138,384)>from()%)+mul(553}what(297,97),&why()why()#!/usr/bin/perl who()who()mul(252,413)} ;when()*what():#mul(178,525)
!,!%*mul(247,765)'from()how()how()/mul(37,480)>who(774,403)^why()why()>{%?;mul(765,805):[%]'mul(555,863),*from(551,599){@mul(680,850)(:@,from()when()!mul$why()*@[}<how()mul(93,798)@mul(73,369)#%mul(213,737))'where():*:mulfrom()#from()&~[why()$;^mul(83,27)where()>&@what()<what()>!/mul(857,84)mul(954,860)]:)what()<}select()#>who()mul(847,874(#who()?, >$/,mul(648,867)!mul(743,614)when()where()from()),where()what()+mul(771,994);/<how()mul(490,823),(@!$}mul(67,164);-;who()mul(367,10),&from()$*mul(720,826)@{mul(788,673)]'}$when()[)-mul;<don't()@how()>&{select()mul(185,672)$?what()?-[mul(462,251)why()['!where()~)why(223,838)do()why()(?];-) mul(320,138)mul(693,135)$from();{%how(){when())mul(641,922)mul(936,690)<don't()-<'?^'&~{where(710,453)mul(82,79)~what()when()/why(581,164)+don't()who()[where()/who(){>~where()mul(405,51):mul(230,693)#^where()[,~%what()mul(847,625)>+:+mul(417,751)what()select()?#$~what()~:mul(136++(;,select()mul(84,993)(%:<{where()mul(804,257),where()(^;mul(699,405)@ {#}what(971,750)][mul(303,720)]how()who()%~who()(!*mul(159,290))*]what()when()!who()?+mul(431,69)>@why()&from()}mul(635,147)^};[mul(751,946)>}mul(750,619)mul(934,728)](where()(])!,>mul(948,935)^what()mul(635,611)mul(154,203)!]${mul(848,181)where()<:'%/why()from()^mul(58,506)mul(289,102)&}mul(219,217)why()mul(815,547)@??~<-?:mul(379,469)where()mul(892,520)@mul(684,138)who(618,612));]how()!select();(mul(620~$:don't()&mul(410,854)[[<@/select()<how()mul(818,694)>[,<mul(577,893)when()select()how())*do()[from() ,what()~mul(97,535);~mul(526,845)/[~}mul(827,275)how()mul(852,966)})]when()-'$mul(894,978)'['>mul(99,314)don't()@:+/how():mul(681,615)%@^'why()why()'*#mul(830,208)who()/<;mul(658,189what()what()mul(830,308)+%; !]!+){mul(545,809)~%when()*)what()/how()mul(113,677)-what()@*mul(881,322)-'mul(676,641)from()mul(793,567)/&+*% '[mul(707,603),what()&what()'$^how()mul(373,428)&]#*%*!when(251,472))how()mul(113,620)&[how(750,572),?+}why()'mul(439,649))-$&{!mul(567,585)how()??<mul(65,148)~%'/mul(591,992)how()mul(897,219):+<$:where()select()>mul(342,193)]mul(939,929){(how()mul(101,869)];^when()*]}^^:mul(490,241)mul(664,51)when()#  }~mul(633,172)@,:>#;>mulfrom()mul(205,672){,#>&mul(102,476)'when()mul(555,446)(:&//[where()/mul(201[when()mul(597,255)<mul(852,830)*what()[why(551,344)!'}<mul(133,373)([-*/^!mul(250,837)]-who(519,492)mul(142,16)who())&>/!-;why()mul(530,146)mul(12,488)%when()~why()?mul(397,506)^>(who(620,254)who(190,64)&]mul(244,996)*where())-*?* :-mul(89,560);#how()*%}}mul(854,585)what()~mul(670,166)*~mul(995,990)how()select()*^where()+~mul(805,731 :$mul(357,136)mul(512,13)~@&)mul(972,955)why()what()}select()how()+))mul(709,220){what()&:*^mul(430,726)!@#++-who()mul(41,47)&,}where(799,279);mul(61,691)+>mul(903,891)}!mul(826,623){>select()^what()><[mul}<^{[what()~}[select();mul(675,405)when()&where()- +?mul(485,232)(where()mul(375,359):]^{<mul(680,7)mulwhere()]mul(608,134)*''^#^mul(309,898)mul(711,574)<+*]}~do()'&-> why()mul(755,634)select()?[who()!where(698,921)*select()where()mul(111,473)when()&}mul(616/%[what()^from()how()^>mul(511,223)select()*when()/-,+what()why()don't()>why()}(+)how()where()when()mul#where()mul(310,171)when()why(){%$>)how()+mul(535,384)['how()^/mul(157,599)]mul(916,257)@when()from(){don't()when()who();; where()^what()how()~mul(668,57)
%+mul(923,967)&(who()from(101,122)why()^mul(327($where()/;/+mul(817,835)?(mul(863,760)#mul(254,722)}+why()mul(399,61)$who()from()where()who()mul(533,810)>:+what(394,757) $mul(252,667)}what()+$mul(342,546)*)*>don't()from(), how()<]^who();mul(629,290)what()mul(234,255)!when(292,811)#from()##why()what()who()don't();mul(869,294)#^><what();~-do()select()~~from()>}mul(372,283)>%why()/<mul(423,194)%mul;[*where()+-)why()mul(573,554)$]/mul(936,543)what()why()-mul(999,520)'@;where()mul(71,216)*mul(344,530:who()*(-+*,mul(74,631)${mul(182,864):^when()~[+mul(618,708)]~mul(634,370))#:#-'/>mul&]]don't()<!mul(609,585)(?(}{<who()/(*mul(301,299):*(,@$:mul(651,306),$why():where()(#];(mul(164,471)* where()who(845,413)select()%,[+>mul(400,866)#]$/(mul(599,385)from()mul(64,766)!:++}!*]mul(256,981)^/:mul(255,258)why()what(940,608);<what()mul(371,4)mul(432,574)select(){mul(921,277)@how()from()mul(913,482),?!@who(),who()&what(23,993)>mul(769,112)*](&'/?%?mul(616,557)/mul(297,516)'why()where()mul(541,625]*@*~<when()<mul(28,891){/when()@-from()@how()mul(38how()'*;mul(909,877)mul(233,412){why()where()% ?+]'where()mul(771,811)+where()(!},?mul(826,359):;mul(604,139))?(how()why()%/;@@mul(686,118)what()who();{}-mul(484,258)when(329,846)}~why() &mul(758,786)/]where()~don't()!why()?[why()mul(909,661)mul(401,973)mul(657,33)what()~$-who()>select()!how()'mul(111,141)!)mul(829,517)#why(103,740)+/mul(147,862)&} ]?what()mul(626,774)?who()when())@(mul(442,344)&select(818,275)where()^*;&mul(935,876)when();what()$/mul(314,497)#)-how()]mul(983,427)why()^how()~~ {mul(864,428)why()<what()/mul(685,901){;who()!'from()>where()mul(218,171)select()who()why()do():~ ~;*:{*mul*select()from(984,969)mul(377,382),'how()-?!>mulwho(),who()how()#mul(479,732)$<;*from()who() mul(371,96)$;(>mul(987,681)who()from())why()do()>who()$-mul(315,10)((:from()*where()@)+mul(279,804)-where(){%when()*mul(680,426)>/@do();mul(68,45) +#(-!#mul(630,936)select()mul(370,414)'-#how(612,677)}don't()),'when()[^?mul(278,179)''^!mul(68,583);&where()[from()''mul(761,143) ~'/ [(@mul,]who()'mul(439,429)>select(508,507)[} mul(585,145){from()>*!<why()mul(661,782)mul(153,516)~$^]select()+mul(108$from()how()what()^^what()]))^mul(416,177)%;<who()>from()why(){mul(427,492)select()mul(460,379)/+where()~;>!,:^mul(8,427)mul(589,356)how(),(@do()from()+when()mul(753,641)~{ +mul(817,120)~;'?>from()%'what()mul(497,124)[)where()$ don't()+what()>+mul(344,301)mul(226,559)<where()/!%;?mul(237,518)-$]%>;)%where()(mul(153,377)'$/*,[*mul(399,320)mul(157,435),^%- >what()@!from()mul(988,920)[@how())where()}$mul(825,518);mul(277,441)select()&from()(?{mul(727,849)< how(226,741)#select()#from()[,'mul(377,916)mul(498,335)[?mul(162,204),;select()~&>^[<mul(999what()!select()[$what()*mul(697,639)what()>'what()#where()when()mul(602,837);why()who()) select()[%mul(26,319)$'&why()+mul(566,508)}?what()>how()@]why()mul(268,280) ;mul(349how()-who())}where():don't()when()<!-)&+[>)mul(261,23)where(629,189)+)~}from()^mul(805,908)from())[::&(:[mul(559,179)$how();how(424,854)?mul(497,761^?}+# mul(73,724)mul(893,324)mul(33,500)@^select()select()%mul(902,772)^do()>]&'* *,mul(531,698+)#where()who()(from()@;}mul(336,925)$what()~why()mul(672,818)who()^from():?[@what()<mul(353,321)how():$(mul(243,816):@ >,'%?+from()mul(710,158)when()?+mul(692,848)+what()/mul(771,109?}how():):from(),>$,don't()?)-when() }mul^who()mul(142,49);select()#mul(36,734)mul(387,920)'%where(821,671)}]mul(639,17)[::select()>>from()mul(335,905){:what(371,338))mul(707,318)
[mul(319,265)<)from()>mul(213,849why()where()-&~{from()where(442,708)how()mul(31,535)mul(835,301)from()-where()/mul(410,971)mul(597,302'~mul(185,542)<@,;select(227,573){[where()mul(781,451),%?who()mul(497,786)<#}select()when(212,649)'mul(633,547)}'+-*~*mul(368,344);>/why()++mul(18,249)>/:[[[{mul(769,663)(;#^*-{who()mul(33,6)@;!&]:<[%mul(984,664)how()where()/$ )mul(163,741)mul(107,247)&]<}^,*$<~mul(475,905)>,>mul(405,913)mul(187,32)mul(505,579)mul(460,119)#^'(mul(478,778){: /?from(13,647){mul'<**mul(505,984)]</!where()+;?mul(574,850){+}from()when()[#?%{mul(46,799)from()(who();from()->!mul(572,718)mul(342,541)@^<select(),mul(318,874?how()mul(243,7)}{mul(187,691@}-what()mul(85,522)~who()<*>mul(335,642)':*{&,don't():what()[* ?who()!mul(202,20),@select()!where(501,423)/mul(986,590)don't()where()when()-where()mul(390,246)when()#!]where()>/%mul(552,289)<$who()<don't()when()-how();<(;mul(514,498)%^?,mul#-+]how()select()mul(533,235)#>{,{@select(251,685)mul(589(~why() ]how()mul(824,521)-!{from()+]who()select()^mul(401,951):+/&$^'[+mul(712,16)&]<}what()&+!~*mul(844,796) }from())mul(122,632>{what()<< what()mul(247,660)(,mul(224,511)<)how();[select()#,mul(389,90)$$&<%where()do()mul(478,963);}<select(){$*[who()/don't() %;mul(473,606)where()%mulwhy()&~&from():>how()@,mul(971,397),!<<<,mul(144,69))what()![what(727,164) (do(){>{#when()/<what()mul(285,888)'+mul(447,749)@[?~how()<#$where()how(895,368)don't()'[%-where()mul(344,170)&@mul(46,591)~ <mul(286,814)select()what()*{#;+*$mul(228,816)from()+',,!what()mul(92,171);'$[~) why()&mul(446,63)+,/+,@>how())mul(720,811)mul(956,682)who()/]$:'%mul(546,971)# why()%&select()[@mul(771,523)'&<when(583,355)mul(172,15)why()^&)what()when()?do()how()mul(600,918)mul(680,387)#mul(701,292)}mul(274,158)who()[+when()mul(731,356)}^,]%:mul(540,292)~ from()mul(142,102)from()/$why() +%mul(141,804)mul(170,292)'mul(907,634)[;;from()how()from()@&mul(46,510)%what(321,677)who()-#@mul(237,342)where()select()@@@#mul(301,339) ^from(718,589)}+/mul(467,190):-mul(392,342)select()^ '*%(mul(259,466)don't()who()?{select()+@what()!*mul(414,247)]>when()when(195,262)*select()>~who()#mul(407,675)&#^,;who()@who()mul(559,443)-:how();,-mul(154,187)-}-what() mul(943,730)$'*/mul(77,492)^mul(67,185)+when()@!where()~$why()<from()mul(308,353)where()+mul(613,527)mul(519,836)'why()mul(943,763)~^when(){,{how()~*mul(15,244)mul(291,737)how()when()&<[)mul(109,238)@where(232,971)where()@why())?~select()mul(48,671)(>mul(30,846);why()@what() {[$mul(228,712))select(888,963)select()-mul(480,617)mul(478,853){, don't()%what(312,885)mul(157,252)/-< %#{[&mul(178,154)#>do())@*how()*mul(567,367)]'what()@mul(949,266))^]what()how()*mul(242,661)$who()mul(77,610)what()where()do();:mul(244,428)mul(924,113)what()~mul(326,797)mul(829,720)/:&who()+@mul(580,849)who(561,730)mul(713,682)from()~;<*$mul(183,921)>^how()who()why()+from()+mul(357,475)*><;}what()'}mul(328,637)> do()<?mul(920,871);+}select(260,714)%:/why()[ don't()->@''[when()mul(424,209)]>>:mul(225,552)when()-select()select()}@[ ]mul(249,678)@# {what()mul(494,553)<from()) who(724,789);/select(839,622)'mul(563,373)]mul(675,891)what()mul(73,441)+?$?!?how()';how()mul(182,313)(]what()"###;
fn main() {
    // let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let re =
        Regex::new(r"mul\((?<lhs>\d{1,3})\,(?<rhs>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();
    let muls = re.captures_iter(input);
    let mut sum: U = 0;
    let mut turn_on = true;
    for mul in muls {
        if mul.name("do").is_some() {
            turn_on = true;
        }
        if mul.name("dont").is_some() {
            turn_on = false;
        }
        if turn_on {
            if let (Some(lhs), Some(rhs)) = (mul.name("lhs"), mul.name("rhs")) {
                sum += lhs.as_str().parse::<U>().unwrap() * rhs.as_str().parse::<U>().unwrap();
            }
        }
    }
    pt.println(sum);
}

mod io {
    use std::collections::{HashSet, VecDeque};
    use std::fmt::Display;
    use std::io::{BufReader, BufWriter, Lines, Read, Write};
    use std::marker::PhantomData;
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct Scanner<R: Read> {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
        lines: Lines<BufReader<R>>,
    }
    impl<R: Read> Scanner<R> {
        pub fn new(source: R) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
                lines: BufReader::new(source).lines(),
            }
        }

        pub fn with_delimiters(source: R, delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
                lines: BufReader::new(source).lines(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let token = loop {
                let front = self.tokens.pop_front();
                if let Some(token) = front {
                    break token;
                }
                self.receive_input();
            };
            token
                .parse::<T>()
                .unwrap_or_else(|_| panic!("input {} isn't a {}", token, type_name::<T>()))
        }

        pub fn next_n<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(self.next());
            }
            v
        }

        pub fn next_line(&mut self) -> String {
            assert!(self.tokens.is_empty(), "You have unprocessed token");
            self.lines
                .next()
                .and_then(|e| e.ok())
                .expect("Failed to read.")
        }

        fn receive_input(&mut self) {
            let line = self
                .lines
                .next()
                .and_then(|e| e.ok())
                .expect("Failed to read.");
            if let Some(delimiters) = &self.delimiters {
                for token in line.split(|c| delimiters.contains(&c)) {
                    self.tokens.push_back(token.to_string());
                }
            } else {
                for token in line.split_whitespace() {
                    self.tokens.push_back(token.to_string());
                }
            }
        }
    }

    pub struct Printer<W: Write> {
        writer: BufWriter<W>,
    }
    impl<W: Write> Printer<W> {
        pub fn new(destination: W) -> Self {
            Self {
                writer: BufWriter::new(destination),
            }
        }

        pub fn print(&mut self, s: impl Display) {
            self.writer
                .write_all(s.to_string().as_bytes())
                .expect("print failed.");
        }

        pub fn print_bytes(&mut self, b: &[u8]) {
            self.writer.write_all(b).expect("print_bytes failed.");
        }

        pub fn println(&mut self, s: impl Display) {
            self.print(s);
            self.newline();
        }

        pub fn newline(&mut self) {
            self.print_bytes(&[b'\n']);
        }

        pub fn print_iter(&mut self, mut iter: impl Iterator<Item = impl Display>) {
            if let Some(e) = iter.next() {
                self.print(&e);
                for e in iter {
                    self.print_bytes(&[b' ']);
                    self.print(&e);
                }
            }
            self.newline();
        }
    }
    impl<W: Write> Drop for Printer<W> {
        fn drop(&mut self) {
            self.writer
                .flush()
                .expect("flush failed when dropping Printer.");
        }
    }
}

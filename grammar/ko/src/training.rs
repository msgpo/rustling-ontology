use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(500.0, Some("KRW"), Precision::Exact), "500원");
    example!(v, check_finance(200.0, Some("$"), Precision::Exact), "200달러");
    example!(v, check_finance(31.0, Some("EUR"), Precision::Exact), "31유로");
    example!(v, check_finance(10.0, Some("£"), Precision::Exact), "10영국파운드");
    example!(v, check_finance(2.0, Some("AUD"), Precision::Exact), "2호주달러");
    example!(v, check_finance(10.0, Some("INR"), Precision::Exact), "10루피");
    example!(v, check_finance(200.25, Some("$"), Precision::Exact), "200달러 25센트");
    example!(v, check_finance(10.0, Some("INR"), Precision::Approximate), "약 10루피");
    example!(v, check_finance(2000.0, Some("$"), Precision::Approximate), "2천 달러쯤");
    example!(v, check_finance(10.0, Some("£"), Precision::Exact), "딱 10파운드");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(37.0, Some("celsius")), "37°C", "섭씨37°", "섭씨37도");
    example!(v, check_temperature(70.0, Some("fahrenheit")), "70°F", "화씨70°", "화씨70도");
    example!(v, check_temperature(45.0, Some("degree")), "45°", "45도");
    example!(v, check_temperature(-15.0, Some("degree")), "영하 15도");
    example!(v, check_temperature(-3.0, Some("degree")), "영하 삼도");
    example!(v, check_temperature(15.0, Some("degree")), "영상 15도");
    example!(v, check_temperature(3.0, Some("degree")), "영상 삼도");
    example!(v, check_temperature(6.0, Some("celsius")), "섭씨 6도");
    example!(v, check_temperature(32.0, Some("fahrenheit")), "화씨 32도");
}

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "방금", "지금");
    example!(v, check_moment!(c, [2013, 2, 12]), "오늘");
    example!(v, check_moment!(c, [2013, 2, 11]), "어제");
    example!(v, check_moment!(c, [2013, 2, 13]), "내일");
    example!(v, check_moment!(c, [2013, 2, 18]), "월요일", "이번주 월요일");
    example!(v, check_moment!(c, [2013, 2, 18]), "2월18일 월요일");
    example!(v, check_moment!(c, [2013, 2, 19]), "화요일", "19일 화요일");
    example!(v, check_moment!(c, [2013, 2, 14]), "목요일");
    example!(v, check_moment!(c, [2013, 2, 15]), "금요일");
    example!(v, check_moment!(c, [2013, 2, 16]), "토요일");
    example!(v, check_moment!(c, [2013, 2, 17]), "일요일");
    example!(v, check_moment!(c, [2013, 3, 1]), "3월 1일");
    example!(v, check_moment!(c, [2013, 3, 3]), "3월 3일");
    example!(v, check_moment!(c, [2015, 3, 3]), "2015년 3월 3일", "이천십오년 삼월 삼일", "2015/3/3", "2015-3-3", "2015-03-03");
    example!(v, check_moment!(c, [2013, 2, 15]), "15일에");
    example!(v, check_moment!(c, [2013, 2, 15]), "2월 15일", "2/15");
    example!(v, check_moment!(c, [2013, 8, 8]), "8월 8일");
    example!(v, check_moment!(c, [2014, 10]), "2014년 10월");
    example!(v, check_moment!(c, [1974, 10, 31]), "1974/10/31", "74/10/31");
    example!(v, check_moment!(c, [2015, 4, 14]), "2015년 4월 14일");
    example!(v, check_moment!(c, [2013, 2, 19]), "다음주 화요일", "다음 화요일");
    example!(v, check_moment!(c, [2014, 3]), "다음 3월");
    example!(v, check_moment!(c, [2013, 2, 18]), "2월 18일 월요일");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "이번주", "금주");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "저번주", "전주");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "다음주", "오는주");
    example!(v, check_moment!(c, [2013, 1]), "저번달");
    example!(v, check_moment!(c, [2013, 3]), "다음달");
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "이번분기");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "다음분기");
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "삼분기");
    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "2018년 4분기");
    example!(v, check_moment!(c, [2012]), "작년");
    example!(v, check_moment!(c, [2013]), "올해");
    example!(v, check_moment!(c, [2014]), "내년");
    example!(v, check_moment!(c, [2013, 2, 10]), "저번주 일요일", "지난주 일요일", "지난 일요일", "저번 일요일");
    example!(v, check_moment!(c, [2013, 2, 5]), "저번주 화요일");
    example!(v, check_moment!(c, [2013, 2, 19]), "다음주 화요일");  //when today is Tuesday, "mardi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 20]), "다음주 수요일");
    example!(v, check_moment!(c, [2013, 2, 22]), "다음주 금요일");
    example!(v, check_moment!(c, [2013, 2, 18]), "이번주 월요일");
    example!(v, check_moment!(c, [2013, 2, 12]), "이번주 화요일");
    example!(v, check_moment!(c, [2013, 2, 13]), "이번주 수요일");
    example!(v, check_moment!(c, [2013, 2, 14]), "내일모레", "모레");
    example!(v, check_moment!(c, [2013, 2, 13, 17]), "내일 저녁다섯시");
    example!(v, check_moment!(c, [2013, 2, 10]), "그제");
    example!(v, check_moment!(c, [2013, 2, 10, 8]), "그제 아침8시", "그제 오전8시");
    example!(v, check_moment!(c, [2013, 3, 25]), "3월 마지막 월요일");
    example!(v, check_moment!(c, [2014, 3, 30]), "2014년 3월 마지막일요일");
    example!(v, check_moment!(c, [2013, 10, 3]), "10월 3일");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "2014년 10월 첫번째주");
    example!(v, check_moment!(c, [2015, 10, 31]), "2015년 10월 마지막날");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "2014년 9월 마지막주");

    // nth of
    example!(v, check_moment!(c, [2013, 10, 1]), "10월 첫째화요일");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "아침 3시", "오전 세시", "새벽 3시");
    example!(v, check_moment!(c, [2013, 2, 12, 3, 18]), "새벽 3시 18분", "새벽 3시 18");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "오후 세시");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 20]), "오후세시이십분", "오후 3시 20분", "15:20", "오후 3시 20");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "오후세시반", "15:30");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 45]), "네시십오분전");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "3:30", "세시반");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 23, 24]), "15:23:24", "세시이십삼분이십사초");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "오늘밤 8시", "저녁 8시");

    // Mixing date and time
    example!(v, check_moment!(c, [2013, 9, 20, 19, 30]), "9월 20일 저녁 7시 30분");
    example!(v, check_moment!(c, [2013, 2, 16, 9]), "토요일 9시");
    example!(v, check_moment!(c, [2014, 7, 18, 19]), "2014년 7월 18일 금요일 오후 7시");
    example!(v, check_moment!(c, [2013, 2, 14]), "내일모레", "모레");
    example!(v, check_moment!(c, [2013, 2, 13, 17]), "내일 저녁다섯시");
    example!(v, check_moment!(c, [2013, 3, 25]), "3월 마지막 월요일");
    example!(v, check_moment!(c, [2014, 3, 30]), "2014년 3월 마지막일요일");
    example!(v, check_moment!(c, [2013, 10, 3]), "10월 3일");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "2014년 10월 첫번째주");
    example!(v, check_moment!(c, [2015, 10, 31]), "2015년 10월 마지막날");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "2014년 9월 마지막주");
    example!(v, check_moment!(c, [2013, 10, 1]), "10월 첫째화요일");
    example!(v, check_moment!(c, [2014, 9, 16]), "2014년 9월 셋째화요일", "2014년 9월 세번째화요일");
    example!(v, check_moment!(c, [2014, 10, 1]), "2014년 10월 첫번째 수요일", "2014년 10월 첫째 수요일");
    example!(v, check_moment!(c, [2014, 10, 8]), "2014년 10월 두번째 수요일", "2014년 10월 둘째 수요일");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "아침 3시", "오전 세시", "새벽 3시");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "오후 세시", "오후 3시");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 20]), "오후세시이십분", "오후 3시 20분", "15:20", "오후 3시 20");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "오후세시반", "15:30");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 45]), "네시십오분전");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "3:30", "세시반");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 23, 24]), "15:23:24", "세시이십삼분이십사초");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "오늘밤 8시", "저녁 8시");
    example!(v, check_moment!(c, [2013, 9, 20, 19, 30]), "9월 20일 저녁 7시 30분");
    example!(v, check_moment!(c, [2013, 2, 16, 9]), "토요일 9시");
    example!(v, check_moment!(c, [2014, 7, 18, 19]), "2014년 7월 18일 금요일 오후 7시");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 4, 30, 1]), "1초안에");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 4, 31, 0]), "일분안에", "일분내에");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 4, 32, 0]), "이분안에", "이분내에");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2013, 2, 12, 5, 30]),"한시간안에", "한시간내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 6, 0, 0]), "한시간반안", "한시간반내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 7, 0, 0]), "두시간반안", "두시간반내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2013, 2, 12, 7, 30]), "몇시간안", "몇시간내");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 30]), "몇시간후");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 7, 30], Direction::After), "몇시간이후");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2013, 2, 13, 4, 30]), "24시간안에", "24시간내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2013, 2, 13, 4]), "하루안에", "하루내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2016, 2]), "삼년안에", "삼년내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2013, 2, 19, 4]), "7일안에", "7일내");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30], [2013, 2, 19]), "1주일안에", "1주일내");
    example!(v, check_moment_span_with_precision!(c, [2013, 2, 12, 4, 30], [2013, 2, 12, 6, 0, 0], Precision::Approximate), "약 한시간반 안에");
    example!(v, check_moment!(c, [2013, 2, 5, 4]), "7일전");
    example!(v, check_moment!(c, [2013, 1, 29, 4]), "14일전", "14일전에");
    example!(v, check_moment!(c, [2013, 1, 22]), "3주전", "3주이전");
    example!(v, check_moment!(c, [2011, 2]), "2년전", "2년이전");
    example!(v, check_moment!(c, [1999]), "99년");
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "이번여름");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "이번겨울");
    example!(v, check_moment!(c, [2013, 12, 25]), "크리스마스");
    example!(v, check_moment!(c, [2013, 12, 24]), "크리스마스이브");
    example!(v, check_moment!(c, [2014, 1, 1]), "신정");
    example!(v, check_moment!(c, [2013, 3, 1]), "삼일절");
    example!(v, check_moment!(c, [2013, 5, 5]), "어린이날");
    example!(v, check_moment!(c, [2013, 6, 6]), "현충일");
    example!(v, check_moment!(c, [2013, 6, 17]), "제헌절");
    example!(v, check_moment!(c, [2013, 8, 15]), "광복절");
    example!(v, check_moment!(c, [2013, 10, 3]), "개천절");
    example!(v, check_moment!(c, [2013, 10, 9]), "한글날");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "오늘저녁");
    example!(v, check_moment_span!(c, [2013, 2, 12, 19], [2013, 2, 13, 00]), "오늘밤");
    example!(v, check_moment_span!(c, [2013, 2, 8, 18], [2013, 2, 11, 00]), "저번주말", "지난주말");
    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "내일저녁");
    example!(v, check_moment_span!(c, [2013, 2, 13, 19], [2013, 2, 14, 00]), "내일밤");
    example!(v, check_moment_span!(c, [2013, 2, 13, 12], [2013, 2, 13, 14]), "내일점심");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "어제저녁");
    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "이번주말");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "월요일 아침");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "2월 15일 아침");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "지난 2초", "지난 이초");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "다음 3초", "다음 삼초");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "지난 2분", "지난 이분");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "다음 3분", "다음 삼분");
    example!(v, check_moment_span!(c, [2013, 2, 12, 3], [2013, 2, 12, 4]), "지난 1시간", "지난 한시간");
    example!(v, check_moment_span!(c, [2013, 2, 11, 4], [2013, 2, 12, 4]), "지난 24시간", "지난 스물네시간");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "다음 3시간");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "지난 2일");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "다음 3일");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "다음 몇일");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11], Grain::Week), "지난 2주", "지난 이주");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11], Grain::Week), "다음 3주", "다음 삼주");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "지난 2달", "지난 두달");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "다음 3달", "다음 세달");
    example!(v, check_moment_span!(c, [2011], [2013]), "지난 2년", "지난 이년");
    example!(v, check_moment_span!(c, [2014], [2017]), "다음 3년", "다음 삼년");
    example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "7월 13일 - 15일", "7월 13일 부터 15일 까지");
    example!(v, check_moment_span!(c, [2013, 8, 8], [2013, 8, 14]), "8월 8일 - 8월 13일", "8월 8일부터 8월 13일까지");
    example!(v, check_moment_span!(c, [2013, 2, 12, 9, 30], [2013, 2, 12, 11, 1]), "9:30 부터 11:00 까지", "9:30 ~ 11:00");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11, 1]), "목요일 9:30 부터 11:00 까지", "목요일 9:30 ~ 11:00");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 12]), "목요일 오전9시 부터 오전11시 까지");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 31]), "11:30-1:30");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "2주 이내에");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 14]), "오후 2시까지");
    example!(v, check_moment!(c, [2013, 2, 12, 14]), "오늘 오후두시에", "오후두시에");
    example!(v, check_moment!(c, [2013, 4, 25, 16]), "4/25 오후4시에");
    example!(v, check_moment!(c, [2013, 2, 13, 15]), "내일 오후 3시");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::After), "오후2시 이후");
    example!(v, check_moment!(c, [2013, 2, 17, 4]), "5일 후");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 11], Direction::Before), "오전11시 전", "오전11시 이전");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "오후에");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 4, 45, 0]), "15분안");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "점심이후");
    example!(v, check_moment!(c, [2013, 2, 12, 10, 30]), "10:30");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "아침");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "오후12시", "정오", "오정", "한낮");
    example!(v, check_moment!(c, [2013, 2, 13, 0]), "오전12시", "자정", "영시");
    example!(v, check_moment!(c, [2013, 3]), "3월", "3월에");

    // Grammar addition
    example!(v, check_moment!(c, [2013, 2, 19, 4]), "7일 후에", "7일 뒤에");
    example!(v, check_moment_with_direction!(c, [2013, 2, 19, 4], Direction::After), "7일 이후에", "7일 이 후에");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "이제");
    example!(v, check_moment!(c, [2013, 2, 11]), "작일", "어저께");
    example!(v, check_moment!(c, [2013, 2, 15]), "돌아오는 금요일", "이번주 금요일");
    example!(v, check_moment!(c, [2013, 3, 5]), "3월의 첫번째 화요일");
    example!(v, check_moment!(c, [2013, 6, 16]), "6월의 3번째 일요일");
    example!(v, check_moment!(c, [2013, 5, 19]), "5월의 둘째 주 일요일");
    example!(v, check_moment!(c, [2013, 2, 12, 19]), "저녁 7시");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 12]), "늦은 아침", "오전 늦게", "아침 늦게", "아침 느지막이");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 16]), "이른 오후", "낮곁", "오후 들어", "오후 일찍");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 19]), "늦은 오후", "오후 늦게");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 12, 21]), "이른 저녁", "초저녁", "저녁 일찍");
    example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 0]), "늦은 저녁", "저녁 늦게");
    example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 0]),  "이른 밤", "밤에 일찍");
    example!(v, check_moment_span!(c, [2013, 2, 12, 6], [2013, 2, 12, 9]), "아침때", "아침 식사때", "아침밥", "조반");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17, 30], [2013, 2, 12, 21]), "저녁", "저녁 식사", "저녁밥");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 14]), "브런취", "브런치", "아침 겸 점심", "늦은 아침", "아점");
    example!(v, check_moment!(c, [2013, 5, 19]), "5월의 둘째 주 일요일");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 5], Direction::After), "5시 되면");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 00]), "2분 있다가", "2분 뒤에", "2분 후");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 00]), "지금부터 2분 후에");
    example!(v, check_moment!(c, [2013, 6, 3]), "6월 3번째 일");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 30]), "현시간부터 3시간 후에");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 35], Grain::Second), "딱 5분 후에");
    example!(v, check_moment_span!(c, [2006], [2013]), "과거 7년간");
    example!(v, check_moment_span!(c, [2014], [2017]), "앞으로 3년간");
    example!(v, check_moment!(c, [2013, 2, 15], Grain::Hour), "오늘보다 3일 후에"); // Unexpected grain

    // Specific day counting
    example!(v, check_moment!(c, [2013, 2, 13, 4], Grain::Hour), "지금부터 하루 후");
    example!(v, check_moment!(c, [2013, 2, 14, 4], Grain::Hour), "지금부터 이틀 후");
    example!(v, check_moment!(c, [2013, 2, 14, 4], Grain::Hour), "지금부터 양일 후");
    example!(v, check_moment!(c, [2013, 2, 15, 4], Grain::Hour), "지금부터 사흘 후");
    example!(v, check_moment!(c, [2013, 2, 16, 4], Grain::Hour), "지금부터 나흘 후");

    // beginning of / end of
    example!(v, check_moment_span!(c, [2013, 7, 1], [2013, 7, 6]), "7월 초에");
    example!(v, check_moment_span!(c, [2013, 7, 25], [2013, 8, 1]), "7월 말에");
    example!(v, check_moment_span!(c, [2014, 3, 1], [2014, 3, 6]), "2014년 3번째 달 초에");
    example!(v, check_moment_span!(c, [2014, 3, 25], [2014, 4, 1]), "2014년 3번째 달 말에");
    example!(v, check_moment_span!(c, [2013, 3, 18], [2013, 3, 20]), "3월의 3번째 주 초에");
    example!(v, check_moment_span!(c, [2013, 3, 22], [2013, 3, 25]), "3월의 3번째 주 말에");
    example!(v, check_moment_span!(c, [2014, 1], [2014, 4]), "2014년 초에");
    example!(v, check_moment!(c, [2011]), "재작년");
    example!(v, check_moment!(c, [2010]), "재재작년");
    example!(v, check_moment!(c, [2016]), "내후년", "명후년", "후후년");
    example!(v, check_moment!(c, [2015]), "후년", "재명년", "내명년");

    // Holidays
    example!(v, check_moment!(c, [2013, 6, 16]), "아버지날");
    example!(v, check_moment!(c, [2013, 5, 12]), "어머니날");
    example!(v, check_moment!(c, [2013, 5, 8]), "어버이날");
    example!(v, check_moment!(c, [2013, 5, 15]), "스승의 날");
    example!(v, check_moment!(c, [2013, 5, 1]), "노동절", "노동일", "근로자의날");
    example!(v, check_moment!(c, [2013, 2, 14]), "발렌타인 데이");
    example!(v, check_moment!(c, [2013, 3, 14]), "화이트 데이");
    example!(v, check_moment!(c, [2013, 5, 20]), "성년의날");
    example!(v, check_moment!(c, [2013, 5, 21]), "부부의날");
    example!(v, check_moment!(c, [2013, 7, 13]), "초복");
    example!(v, check_moment!(c, [2013, 7, 23]), "중복");
    example!(v, check_moment!(c, [2013, 8, 12]), "말복");
    example!(v, check_moment!(c, [2013, 10, 31]), "핼러윈", "핼러윈데이");
    example!(v, check_moment!(c, [2013, 10, 1]), "국군의날");
    example!(v, check_moment!(c, [2013, 10, 2]), "노인의날");
    example!(v, check_moment!(c, [2013, 10, 25]), "독도의 날");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "0", "영", "빵", "공");
    example!(v, check_integer(1), "1", "일", "하나", "한");
    example!(v, check_integer(10), "10", "십", "열");
    example!(v, check_integer(11), "11", "십일", "열하나", "십하나", "열한");
    example!(v, check_integer(20), "20", "이십", "스물");
    example!(v, check_integer(35), "35", "삼십오", "서른다섯");
    example!(v, check_integer(47), "47", "사십칠", "마흔일곱");
    example!(v, check_integer(52), "52", "오십이", "쉰둘", "쉰두");
    example!(v, check_integer(69), "69", "육십구", "예순아홉");
    example!(v, check_integer(71), "71", "칠십일", "일흔하나", "일흔한");
    example!(v, check_integer(84), "84", "팔십사", "여든넷");
    example!(v, check_integer(93), "93", "구십삼", "아흔셋");
    example!(v, check_integer(100), "100", "백");
    example!(v, check_integer(123), "123", "백이십삼");
    example!(v, check_integer(579), "579", "오백칠십구");
    example!(v, check_integer(1000), "1000", "천");
    example!(v, check_integer(1723), "1723", "천칠백이십삼");
    example!(v, check_integer(5619), "5619", "오천육백십구");
    example!(v, check_integer(10000), "10000", "만", "일만");
    example!(v, check_integer(12345), "12345", "만이천삼백사십오", "일만이천삼백사십오");
    example!(v, check_integer(58194), "58194", "오만팔천백구십사");
    example!(v, check_integer(581900), "581900", "오십팔만천구백");
    example!(v, check_integer(5819014), "5819014", "오백팔십일만구천십사");
    example!(v, check_integer(58190148), "58190148", "오천팔백십구만백사십팔");
    example!(v, check_integer(100000000), "100000000", "일억");
    example!(v, check_integer(274500000000), "274500000000", "이천칠백사십오억");
    example!(v, check_integer(100000002), "100000002", "일억이");
    example!(v, check_integer(27350000), "27350000", "이천칠백삼십오만");
    example!(v, check_integer(3235698120), "3235698120", "삼십이억삼천오백육십구만팔천백이십");
    example!(v, check_integer(40234985729), "40234985729", "사백이억삼천사백구십팔만오천칠백이십구");
    example!(v, check_integer(701239801123), "701239801123", "칠천십이억삼천구백팔십만천백이십삼");
    example!(v, check_float(3.4), "3.4", "삼점사");
    example!(v, check_float(4123.3), "4123.3", "사천백이십삼점삼");
    example!(v, check_float(1.23), "일점이삼");
    example!(v, check_integer(-3), "-3", "마이너스3", "마이너스삼", "마이너스 3", "마이나스3", "마이나스 3");
    example!(v, check_float(3.0 / 4.0), "3/4", "사분의삼");
    example!(v, check_ordinal(25), "스물다섯번째", "이십오번째");
    example!(v, check_ordinal(1), "첫번째", "첫째", "첫번", "첫");
}

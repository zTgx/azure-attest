use azure_core::auth::{AccessToken, Secret, TokenCredential};
use azure_core::date;
use azure_svc_attestation::models::{InitTimeData, RuntimeData};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use time::macros::date;

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use hex;
use time::OffsetDateTime;
use azure_svc_attestation::ClientBuilder;
use azure_svc_attestation::models::AttestSgxEnclaveRequest;
use tokio::runtime::Builder;
use azure_core::Url;

#[derive(Serialize, Deserialize)]
struct AttestationResult {
    #[serde(rename = "isDebuggable")]
    is_debuggable: bool,
    #[serde(rename = "isValid")]
    is_valid: bool,
    #[serde(rename = "version")]
    version: String,
}

#[derive(Debug)]
pub(crate) struct MockCredential;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for MockCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        Ok(AccessToken::new(
            "TOKEN".to_owned(),
            OffsetDateTime::now_utc() + date::duration_from_days(14),
        ))
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        Ok(())
    }
}

fn main() {
    let endpoint = "https://testazureprovider.eus.attest.azure.net";
    let quote = intel_sgx_quote_data();
    println!("quote len: {}", quote.len());

    let builder = ClientBuilder::new(Arc::new(MockCredential)).endpoint(Url::from_str(endpoint).unwrap());

    let client =  builder.build().unwrap();
    let client = client.attestation_client();

    let mut request = AttestSgxEnclaveRequest::new();
    request.quote = Some(intel_sgx_quote_data());

    let mut rundata = RuntimeData::new();
    rundata.data = Some("010203040506".to_string());
    rundata.data_type = Some(azure_svc_attestation::models::DataType::Binary);

    request.runtime_data = Some(rundata);
    // request.init_time_data = Some(InitTimeData::new());
    // request.draft_policy_for_attestation = None;
    // request.nonce = None;

    let request_builder = client.attest_sgx_enclave(request);

    let rt = Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()
    .unwrap();

    rt.block_on(async {
        let response = request_builder.send().await;

        println!("response: {:#?}", response);    
    });

    // let attestation_result = response.json().unwrap();

    // println!("Is Debuggable: {}", attestation_result.is_debuggable);
    // println!("Is Valid: {}", attestation_result.is_valid);
    // println!("Version: {}", attestation_result.version);
}

fn intel_sgx_quote_data() -> String {
    let mut file = File::open("quote.dat").expect("Failed to open file");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let hex_string = hex::encode(buffer);

    // println!("{}", hex_string);

    let hex_string = "030002000000000005000A00939A7233F79C4CA9940A0DB3957F0607632F3C4E364B941D8FFC70AE65715B9C000000000F0F0305FF8006000000000000000000000000000000000000000000000000000000000000000000000000000000000005000000000000000700000000000000B30F4E57A1BFE032174DEFB56AA8467C962288F1028297DB908577CDDEF42632000000000000000000000000000000000000000000000000000000000000000068D0154819AD17AD4EBA30ECA03ED26D0FD71EF96F5704CE30BB488327C5F639000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007192385C3C0605DE55BB9476CE1D90748190ECB32A8EED7F5207B30CF6A1FE89000000000000000000000000000000000000000000000000000000000000000030100000267982041E37505F6AA0EC479607DC37C08F21C482208D7018AD0D52C86838AF1408497E4AA20CA3403E6A626A78C5C7DC5ECBF4071E4D899724C17A40C2D8BDDD9F96249AFF26D2F76C95D078CCC7370CC4A746868EC7BBE3C7458366C40190B1C8E0CE189274CBC58260F2A4E3E9D89C3B63BFE6EADEE3A1AA65C14441E5C90F0F0305FF800600000000000000000000000000000000000000000000000000000000000000000000000000000000001500000000000000070000000000000060D85AF28BE8D1C40A08D98B009D5F8ACC1384A385CF460800E478791D1A979C00000000000000000000000000000000000000000000000000000000000000008C4F5775D796503E96137F77C68A829A0056AC8DED70140B081B094490C57BFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000042A91B281E284ED23257780F00009E7FAF4550B337E77B192A1797C63FB29CDF0000000000000000000000000000000000000000000000000000000000000000164EEB1037871B0DBC9F6E4747250E898E019E10DBB11DD332618F0B537ACECF3D545E92D7A14B55ECBA0F29C49BE5A3B2900287ECFB2CB9A8F9FCD42AAE846E2000000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F0500C80D00002D2D2D2D2D424547494E2043455254494649434154452D2D2D2D2D0A4D494945667A43434243616741774942416749554D4546754E6A7A583877303350546E45426C487261454C776A495177436759494B6F5A497A6A3045417749770A6354456A4D4345474131554541777761535735305A577767553064594946424453794251636D396A5A584E7A6233496751304578476A415942674E5642416F4D0A45556C756447567349454E76636E4276636D4630615739754D5251774567594456515148444174545957353059534244624746795954454C4D416B47413155450A4341774351304578437A414A42674E5642415954416C56544D423458445449774D4459784D7A417A4D4455784E466F58445449334D4459784D7A417A4D4455780A4E466F77634445694D434147413155454177775A535735305A5777675530645949464244537942445A584A3061575A70593246305A5445614D426747413155450A43677752535735305A577767513239796347397959585270623234784644415342674E564241634D43314E68626E526849454E7359584A684D517377435159440A5651514944414A445154454C4D416B474131554542684D4356564D775754415442676371686B6A4F5051494242676771686B6A4F50514D4242774E43414152500A766D2B39433677422F466E554A6C714D73674D6A776C3847524763526C61696532742B64663677567176486E697A354765466B356478726867784A2F336E47520A594B634D6C4F4F4656662B4F51496931575634756F3449436D7A434341706377487759445652306A42426777466F4155304F6971326E58582B53354A463567380A6578526C304E587957553077587759445652306642466777566A42556F464B6755495A4F6148523063484D364C79396863476B7564484A316333526C5A484E6C0A636E5A705932567A4C6D6C75644756734C6D4E766253397A5A3367765932567964476C6D61574E6864476C76626939324D6939775932746A636D772F593245390A63484A765932567A633239794D42304741315564446751574242516B757235572F455077672F304731324D543373466E634C56614B6A414F42674E56485138420A4166384542414D434273417744415944565230544151482F42414977414443434164514743537147534962345451454E4151534341635577676748424D4234470A43697147534962345451454E415145454543772B4154526B3852494D3539487A37745A46772F41776767466B42676F71686B69472B453042445145434D4949420A5644415142677371686B69472B4530424451454341514942447A415142677371686B69472B4530424451454341674942447A415142677371686B69472B4530420A4451454341774942416A415142677371686B69472B45304244514543424149424244415142677371686B69472B453042445145434251494241544152426773710A686B69472B4530424451454342674943414941774541594C4B6F5A496876684E4151304241676343415159774541594C4B6F5A496876684E41513042416767430A415141774541594C4B6F5A496876684E4151304241676B43415141774541594C4B6F5A496876684E4151304241676F43415141774541594C4B6F5A496876684E0A4151304241677343415141774541594C4B6F5A496876684E4151304241677743415141774541594C4B6F5A496876684E4151304241673043415141774541594C0A4B6F5A496876684E4151304241673443415141774541594C4B6F5A496876684E4151304241673843415141774541594C4B6F5A496876684E41513042416841430A415141774541594C4B6F5A496876684E415130424168454341516F774877594C4B6F5A496876684E4151304241684945454138504167514267415941414141410A41414141414141774541594B4B6F5A496876684E4151304241775143414141774641594B4B6F5A496876684E4151304242415147414A4275315141414D4138470A43697147534962345451454E4151554B41514177436759494B6F5A497A6A30454177494452774177524149674449546E614C5372713475504D62614E553842540A3370386A5A3678467A46744D393631576E625075374E344349445671396C4E36436137722F35585844454266624F61664C58543774393653307271356F7741300A664337560A2D2D2D2D2D454E442043455254494649434154452D2D2D2D2D0A2D2D2D2D2D424547494E2043455254494649434154452D2D2D2D2D0A4D4949436C7A4343416A36674177494241674956414E446F71747031312F6B7553526559504873555A644456386C6C4E4D416F4743437147534D343942414D430A4D476778476A415942674E5642414D4D45556C756447567349464E48574342536232393049454E424D526F77474159445651514B4442464A626E526C624342440A62334A7762334A6864476C76626A45554D424947413155454277774C553246756447456751327868636D4578437A414A42674E564241674D416B4E424D5173770A435159445651514745774A56557A4165467730784F4441314D6A45784D4451314D4468614677307A4D7A41314D6A45784D4451314D4468614D484578497A41680A42674E5642414D4D476B6C756447567349464E48574342515130736755484A765932567A6332397949454E424D526F77474159445651514B4442464A626E526C0A6243424462334A7762334A6864476C76626A45554D424947413155454277774C553246756447456751327868636D4578437A414A42674E564241674D416B4E420A4D517377435159445651514745774A56557A425A4D424D4742797147534D34394167454743437147534D34394177454841304941424C39712B4E4D7032494F670A74646C31626B2F75575A352B5447516D38614369387A373866732B664B435133642B75447A586E56544154325A68444369667949754A77764E33774E427039690A484253534D4A4D4A72424F6A6762737767626777487759445652306A42426777466F4155496D554D316C71644E496E7A6737535655723951477A6B6E427177770A556759445652306642457377535442486F45576751345A426148523063484D364C79396A5A584A3061575A70593246305A584D7564484A316333526C5A484E6C0A636E5A705932567A4C6D6C75644756734C6D4E766253394A626E526C62464E4857464A76623352445153356A636D7777485159445652304F42425945464E446F0A71747031312F6B7553526559504873555A644456386C6C4E4D41344741315564447745422F77514541774942426A415342674E5648524D4241663845434441470A4151482F416745414D416F4743437147534D343942414D43413063414D45514349432F396A2B3834542B487A74564F2F734F5142574A6253642B2F327565784B0A342B6141306A6346424C63704169413364684D72463563443532743646714D764149706A385864476D79326265656C6A4C4A4B2B707A706352413D3D0A2D2D2D2D2D454E442043455254494649434154452D2D2D2D2D0A2D2D2D2D2D424547494E2043455254494649434154452D2D2D2D2D0A4D4949436A6A4343416A53674177494241674955496D554D316C71644E496E7A6737535655723951477A6B6E42717777436759494B6F5A497A6A3045417749770A614445614D4267474131554541777752535735305A5777675530645949464A766233516751304578476A415942674E5642416F4D45556C756447567349454E760A636E4276636D4630615739754D5251774567594456515148444174545957353059534244624746795954454C4D416B47413155454341774351304578437A414A0A42674E5642415954416C56544D423458445445344D4455794D5445774E4445784D566F5844544D7A4D4455794D5445774E4445784D466F77614445614D4267470A4131554541777752535735305A5777675530645949464A766233516751304578476A415942674E5642416F4D45556C756447567349454E76636E4276636D46300A615739754D5251774567594456515148444174545957353059534244624746795954454C4D416B47413155454341774351304578437A414A42674E56424159540A416C56544D466B77457759484B6F5A497A6A3043415159494B6F5A497A6A3044415163445167414543366E45774D4449595A4F6A2F69505773437A61454B69370A314F694F534C52466857476A626E42564A66566E6B59347533496A6B4459594C304D784F346D717379596A6C42616C54565978465032734A424B357A6C4B4F420A757A43427544416642674E5648534D4547444157674251695A517A575770303069664F44744A5653763141624F5363477244425342674E5648523845537A424A0A4D45656752614244686B466F64485277637A6F764C324E6C636E52705A6D6C6A5958526C63793530636E567A6447566B63325679646D6C6A5A584D75615735300A5A577775593239744C306C756447567355306459556D397664454E424C6D4E796244416442674E564851344546675155496D554D316C71644E496E7A673753560A55723951477A6B6E4271777744675944565230504151482F424151444C39712B4E4D7032494F670A74646C31626B2F75575A352B5447516D38614369387A373866732B664B435133642B75447A586E56544154325A68444369667949754A77764E33774E427039690A484253534D4A4D4A72424F6A6762737767626777487759445652306A42426777466F4155496D554D316C71644E496E7A6737535655723951477A6B6E427177770A556759445652306642457377535442486F45576751345A426148523063484D364C79396A5A584A3061575A70593246305A584D7564484A316333526C5A484E6C0A636E5A705932567A4C6D6C75644756734C6D4E766253394A626E526C62464E4857464A76623352445153356A636D7777485159445652304F42425945464E446F0A71747031312F6B7553526559504873555A644456386C6C4E4D41344741315564447745422F77514541774942426A415342674E5648524D4241663845434441470A4151482F416745414D416F4743437147534D343942414D43413063414D45514349432F396A2B3834542B487A74564F2F734F5142574A6253642B2F327565784B0A342B6141306A6346424C63704169413364684D72463563443532743646714D764149706A385864476D79326265656C6A4C4A4B2B707A706352413D3D0A2D2D2D2D2D454E442043455254494649434154452D2D2D2D2D0A2D2D2D2D2D424547494E2043455254494649434154452D2D2D2D2D0A4D4949436A6A4343416A53674177494241674955496D554D316C71644E496E7A6737535655723951477A6B6E42717777436759494B6F5A497A6A3045417749770A614445614D4267474131554541777752535735305A5777675530645949464A766233516751304578476A415942674E5642416F4D45556C756447567349454E760A636E4276636D4630615739754D5251774567594456515148444174545957353059534244624746795954454C4D416B47413155454341774351304578437A414A0A42674E5642415954416C56544D423458445445344D4455794D5445774E4445784D566F5844544D7A4D4455794D5445774E4445784D466F77614445614D4267470A4131554541777752535735305A5777675530645949464A766233516751304578476A415942674E5642416F4D45556C756447567349454E76636E4276636D46300A615739754D5251774567594456515148444174545957353059534244624746795954454C4D416B47413155454341774351304578437A414A42674E56424159540A416C56544D466B77457759484B6F5A497A6A3043415159494B6F5A497A6A3044415163445167414543366E45774D4449595A4F6A2F69505773437A61454B69370A314F694F534C52466857476A626E42564A66566E6B59347533496A6B4459594C304D784F346D717379596A6C42616C54565978465032734A424B357A6C4B4F420A757A43427544416642674E5648534D4547444157674251695A517A575770303069664F44744A5653763141624F5363477244425342674E5648523845537A424A0A4D45656752614244686B466F64485277637A6F764C324E6C636E52705A6D6C6A5958526C63793530636E567A6447566B63325679646D6C6A5A584D75615735300A5A577775593239744C306C756447567355306459556D397664454E424C6D4E796244416442674E564851344546675155496D554D316C71644E496E7A673753560A55723951477A6B6E4271777744675944565230504151482F42415144416745474D42494741315564457745422F7751494D4159424166384341514577436759490A4B6F5A497A6A30454177494453414177525149675151732F30387279636450617543466B3855505158434D416C736C6F4265374E7761514754636470613045430A495143557438534776784B6D6A70634D2F7A3057503944766F3868326B3564753169574464426B416E2B306969413D3D0A2D2D2D2D2D454E442043455254494649434154452D2D2D2D2D0A00".to_string();
    hex_string
}
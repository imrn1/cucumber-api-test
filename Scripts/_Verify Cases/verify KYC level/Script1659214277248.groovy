import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.logging.KeywordLogger as KeywordLogger


static boolean isNullOrEmpty(String str) { return (str == null || str.allWhitespace) }

static int KYCLevel(String kycLevel){
	
	switch(kycLevel) {
		case "Unknown":
			return 10;
		case "Unverified":
			return 20;
		case "Verified":
			return 30;
		case "Contracted":
			return 40;
	}
}


if(!isNullOrEmpty("$expected_kyc_level")) {
	def kycLevel = 0;
	
	if("$account_type"== "1") {
		// account_type = 1 => personal account
		def GetPersonalAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Get Personal Account', [('account_number') : "$account_number"]))
		kycLevel = WS.getElementPropertyValue(GetPersonalAccountResponse, "payload.kyc_level")
	}
	else if("$account_type"=="2") {
		// account_type = 2 => business account
		def GetBusinessAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Get Business Account', [('account_number') : "$account_number"]))
		kycLevel = WS.getElementPropertyValue(GetBusinessAccountResponse, "payload.kyc_level")
	}
	else {
		println("geçersiz account_type")
		KeywordLogger logger = new KeywordLogger()
	 	logger.logWarning("geçersiz account_type")
		WS.verifyNotEqual("$account_type", "-1")  //  hata vermesi için
	}
	
	if(kycLevel != 0)
		WS.verifyEqual(KYCLevel(kycLevel),"$expected_kyc_level")
}
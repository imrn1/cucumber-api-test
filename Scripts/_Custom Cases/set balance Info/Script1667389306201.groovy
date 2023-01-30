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

import com.kms.katalon.core.testobject.ResponseObject
import org.json.JSONObject;

static boolean isNullOrEmpty(String str) {
	return (str == null) || str.allWhitespace
}

if( (!isNullOrEmpty("${wallet_number}") && "${wallet_number}" !="123" ) && (!isNullOrEmpty("${account_number}") && "${account_number}" !="123" ))
{
	def walletInfoResponse = WS.sendRequestAndVerify(findTestObject('Postman/Wallet/WalletInfo',
		[
			"wallet_number":"$wallet_number",
			"account_number": "$account_number"
		]))
	
	// wallet info response'u işlem durumuna göre global değişkenlere atanması
	if("$beforeTransaction" == '1') {
		
		println("before transaction")
		if("$is_sender" == '1') {
			println("sender wallet ınfo")
			GlobalVariable.beforeTransactionSenderWalletInfo = walletInfoResponse
		}else {
			println("receiver wallet ınfo")
			GlobalVariable.beforeTransactionReceiverWalletInfo = walletInfoResponse
		}
		
	}else {
		
		println("after transaction")
		if("$is_sender" == '1') {
			println("sender wallet ınfo")
			GlobalVariable.afterTransactionSenderWalletInfo = walletInfoResponse
		}else {
			println("receiver wallet ınfo")
			GlobalVariable.afterTransactionReceiverWalletInfo = walletInfoResponse
		}
	}
	
	
	if("$write") {
		// tüm durumlara ait wallet bilgisi alınmış - dosyaya yazdırılabilir
		
		println("dosyaya yazdır")
		try {
			if((ResponseObject)GlobalVariable.beforeTransactionSenderWalletInfo != null) {
			//	json = new JSONObject( ((ResponseObject)GlobalVariable.beforeTransactionSenderWalletInfo).getBodyContent().getText());
				
				
				json = JSON.parse( ((ResponseObject)GlobalVariable.beforeTransactionSenderWalletInfo).getBodyContent().getText() )
				
				println(json.toString(4) + "\n");
				println(json.getDouble("payload.wallet_info.total_balance"));
			    
				
			}
		}catch(Exception ex){
				println(((ResponseObject)GlobalVariable.beforeTransactionSenderWalletInfo).getBodyContent().getText());
		}
		
		
		 
		
		
		
		
		
		
	
	
		
	}
	
	/*
	def fileName =  "işlem öncesi WalletInfo - "+"$serviceName"
	def beforeProcess = "islem oncesi\n"
	if("$beforeTransaction" != '1') {
		beforeProcess = "islem sonrasi\n"
		fileName =  "işlem sonrası WalletInfo - "+"$serviceName"
		
	*/
	
}
	
	
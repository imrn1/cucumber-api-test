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









import customKeyword.Verify as Verify
import customKeyword.CustomMethods as Custom
import com.kms.katalon.core.testobject.ResponseObject





String body = """{
 
	"account_number": '${GlobalVariable.pre_account_number}${Custom.isServiceCall("$account_number")}',
	
		"national_id": ${Custom.isServiceCall("$national_id")},
		"first_name": ${Custom.isServiceCall("$first_name")},
		"last_name": ${Custom.isServiceCall("$last_name")},
		"birth_year": ${Custom.isServiceCall("$birth_year")},
		"birth_day": ${Custom.isServiceCall("$birth_day")},
		"birth_month": ${Custom.isServiceCall("$birth_month")},
		"national_document_type_id": ${Custom.isServiceCall("$national_document_type_id")},
		"country_code": ${Custom.isServiceCall("$country_code")},
		
		"sector_id": ${Custom.isServiceCall("$sector_id")},
		"address": ${Custom.isServiceCall("$address")},
		"province_id": ${Custom.isServiceCall("$province_id")},
		"city": ${Custom.isServiceCall("$city")}

}"""

println("body : "+body)

int i = 0
while(true) {
	if(i==5) {
		break
	}
	
	
	body = """{
	    "account_number": '${GlobalVariable.pre_account_number}${Custom.isServiceCall("$account_number")}',
		"national_id": ${Custom.isServiceCall("$national_id")},
		"first_name": ${Custom.isServiceCall("$first_name")},
		"last_name": ${Custom.isServiceCall("$last_name")},
		"birth_year": ${Custom.isServiceCall("$birth_year")},
		"birth_day": ${Custom.isServiceCall("$birth_day")},
		"birth_month": ${Custom.isServiceCall("$birth_month")},
		"national_document_type_id": ${Custom.isServiceCall("$national_document_type_id")},
		"country_code": ${Custom.isServiceCall("$country_code")},
		"sector_id": ${Custom.isServiceCall("$sector_id")},
		"address": ${Custom.isServiceCall("$address")},
		"province_id": ${Custom.isServiceCall("$province_id")},
		"city": ${Custom.isServiceCall("$city")}
}"""
	println(body)
	
	
	i++
}




/*
ResponseObject response = Custom.sendRequest('/v1/Account/SubmitKYCForm', body, "POST",
	"$process_description", "$expected_status", "$expected_code", "$expected_message",
	"$expected_ResponseStatusCode", "SubmitKYCForm",  true )


//ResponseObject response = Custom.sendRequest('/v1/Account/GetPersonalAccount', body)
 
Verify.VerifyResponseStatusCode("$expected_ResponseStatusCode",response)
Verify.VerifyElementPropetyValue("$expected_message", "message", response)
Verify.VerifyElementPropetyValue("$expected_status", "status", response)
Verify.VerifyElementPropetyValue("$expected_code", "code", response)

*/











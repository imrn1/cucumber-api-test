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
	'account_number': '${GlobalVariable.pre_account_number}${Custom.isServiceCall("$account_number")}' ,
    "currency_code": ${Custom.isServiceCall("$currency_code")},
    "alias": ${Custom.isServiceCall("$alias")},
    "use_iban": ${Custom.isServiceCall("$use_iban")},
    "user_number": ${Custom.isServiceCall("$user_number")},
    "group_code": ${Custom.isServiceCall("$group_code")},
    "user_first_name": ${Custom.isServiceCall("$user_first_name")},
    "user_last_name": ${Custom.isServiceCall("$user_last_name")},
    "user_phone_country_code": ${Custom.isServiceCall("$user_phone_country_code")},
    "user_phone_number": ${Custom.isServiceCall("$user_phone_number")},
    "user_email": '${GlobalVariable.pre_account_number}${Custom.isServiceCall("$user_email")}',  
    "contact_address":{
        "first_name": ${Custom.isServiceCall("$first_name")},
        "last_name": ${Custom.isServiceCall("$last_name")},
        "email": ${Custom.isServiceCall("$email")},
        "address_line1": ${Custom.isServiceCall("$address_line1")},
        "address_line2": ${Custom.isServiceCall("$address_line2")},
        "zip_postal_code": ${Custom.isServiceCall("$zip_postal_code")},
        "phone_number": ${Custom.isServiceCall("$phone_number")},
        "state_province_code": ${Custom.isServiceCall("$state_province_code")},
        "country_code": ${Custom.isServiceCall("$country_code")}
    }
}"""


println("body : "+body)



ResponseObject response = Custom.sendRequest('/v1/Account/RegisterPersonalAccount', body, "POST",
	"$process_description", "$expected_status", "$expected_code", "$expected_message",
	"$expected_ResponseStatusCode", "RegisterPersonalAccount",  true )


//ResponseObject response = Custom.sendRequest('/v1/Account/GetPersonalAccount', body)
 
Verify.VerifyResponseStatusCode("$expected_ResponseStatusCode",response)
Verify.VerifyElementPropetyValue("$expected_message", "message", response)
Verify.VerifyElementPropetyValue("$expected_status", "status", response)
Verify.VerifyElementPropetyValue("$expected_code", "code", response)



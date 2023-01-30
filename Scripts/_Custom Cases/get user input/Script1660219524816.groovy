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


BufferedReader br = new BufferedReader(new InputStreamReader(System.in))

/* 
GlobalVariable.user_input = br.readLine()
*/

println("array : "+ "$arrayName" +" class : "+ "$arrayName".getClass())
println("value : "+ "$valueName" +" class : "+ "$valueName".getClass())



 
println('Input:  '+ "$description" + '?\n')

if("$arrayName" == "null") { // value ataması yapılmalı
	GlobalVariable["$valueName"] = br.readLine();
	println("value ataması : "+GlobalVariable["$valueName"])
}
else {
	if(Objects.isNull(GlobalVariable["$arrayName"])) {
		println('GlobalVariable[' +"$arrayName" +'] = null ')
		
	    GlobalVariable["$arrayName"] = new HashMap<>() 
		GlobalVariable["$arrayName"].put("$description",br.readLine())
		//println("array ilk atama : "+GlobalVariable["$arrayName"][0])
		println("array ilk atama : "+GlobalVariable["$arrayName"].get("$description"))
	}else {
		GlobalVariable["$arrayName"].put("$description",br.readLine())
		int size = GlobalVariable["$arrayName"].size()
		println("array size : "+ size +"  array  atama : "+GlobalVariable["$arrayName"].get("$description"))
	}
	
	 /////////////
	for(def x : GlobalVariable["$arrayName"]) {
		println("item : "+x)
	}
} 
	 
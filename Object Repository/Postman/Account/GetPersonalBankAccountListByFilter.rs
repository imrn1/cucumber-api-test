<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetPersonalBankAccountListByFilter</name>
   <tag></tag>
   <elementGuidId>eddf0323-8a5d-4988-b04f-0888ac3add3e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;page_size\&quot;: \&quot;${page_size}\&quot;,\n  \&quot;page_index\&quot;: \&quot;${page_index}\&quot;,\n  \&quot;order_column\&quot;:\&quot;${order_column}\&quot;,\n  \&quot;order_by\&quot;: \&quot;${order_by}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d4514f69-e541-4829-ab15-8cc7ce300a03</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>644ede29-952f-433f-be88-52dd8f5c6665</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/GetPersonalBankAccountListByFilter</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>c5df72e5-cf48-4484-b781-8fca380ca4e7</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>9f9c360f-1301-42ea-91ba-d0a4f0658aca</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c1dfccb8-6cb7-4802-b12d-71e3fa77a19d</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>3b3a43d9-9b19-4754-adf6-8db1f7975e03</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>1b2b1307-3232-420d-ab9c-deb2757ccc4c</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>8d4b86e4-870b-4985-80e8-da23e4113239</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>1d99695b-a7ec-4061-8a15-344cc8473fb9</id>
      <masked>false</masked>
      <name>order_by</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


GlobalVariable.responseObject = response
GlobalVariable.requestObject = request


println(&quot;GetPersonalBankAccountListByFilter request body: \n&quot;+ request.getBodyContent().getText())

if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}
import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;GetPersonalBankAccountListByFilter response body\n&quot;+json.toString(4)); // Print it with specified indentation
 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
